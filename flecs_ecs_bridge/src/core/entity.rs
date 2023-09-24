use std::{ops::Deref, os::raw::c_void};

use super::{
    c_binding::bindings::{
        ecs_add_id, ecs_clear, ecs_delete, ecs_get_world, ecs_has_id, ecs_new_w_id, ecs_remove_id,
        EcsExclusive, EcsWildcard,
    },
    c_types::{EntityT, IdT, WorldT},
    component::{CachedComponentData, ComponentType, Enum, Struct},
    entity_view::EntityView,
    enum_type::CachedEnumData,
    id::Id,
    utility::functions::ecs_pair,
    utility::macros::*,
};

pub struct Entity {
    entity_view: EntityView,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            entity_view: EntityView::default(),
        }
    }
}

impl Deref for Entity {
    type Target = EntityView;

    fn deref(&self) -> &Self::Target {
        &self.entity_view
    }
}

impl Entity {
    /// Create new entity.
    /// ### Safety
    /// This function is unsafe because it assumes that the world is not null.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(world: *mut WorldT) -> Self {
        Self {
            entity_view: EntityView::new_from_existing(world, unsafe { ecs_new_w_id(world, 0) }),
        }
    }

    /// Wrap an existing entity id.
    /// # Arguments
    /// * `world` - The world the entity belongs to.
    /// * `id` - The entity id.
    pub fn new_from_existing(world: *mut WorldT, id: IdT) -> Self {
        Self {
            entity_view: EntityView::new_from_existing(world, id),
        }
    }

    // Explicit conversion from flecs::entity_t to Entity
    pub const fn new_only_id(id: EntityT) -> Self {
        Self {
            entity_view: EntityView::new_only_id(id),
        }
    }

    pub fn add_component_id(self, component_id: IdT) -> Self {
        unsafe { ecs_add_id(self.world, self.raw_id, component_id) }
        self
    }

    pub fn add_component<T: CachedComponentData>(self) -> Self {
        let world = self.world;
        self.add_component_id(T::get_id(world))
    }

    pub fn add_pair_from_ids(self, id: EntityT, id2: EntityT) -> Self {
        let world = self.world;
        self.add_component_id(ecs_pair(id, id2))
    }

    pub fn add_pair<T, U>(self) -> Self
    where
        T: CachedComponentData,
        U: CachedComponentData + ComponentType<Struct>,
    {
        let world = self.world;
        self.add_pair_from_ids(T::get_id(world), U::get_id(world))
    }

    /// Adds a pair to the entity composed of a tag and an enum constant.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The tag (first element of the pair).
    /// - `U`: The enum constant (second element of the pair).
    ///
    /// # Parameters
    ///
    /// - `enum_value`: The enum constant.
    ///
    /// # Returns
    ///
    /// Returns the updated entity.
    pub fn add_enum_tag<T, U>(self, enum_value: U) -> Self
    where
        T: CachedComponentData,
        U: CachedComponentData + ComponentType<Enum> + CachedEnumData,
    {
        let world = self.world;
        self.add_pair_from_ids(
            T::get_id(world),
            enum_value.get_entity_id_from_enum_field(world),
        )
    }

    /// Adds a pair to the entity where the first element is the enumeration type,
    /// and the second element is the enumeration constant.
    ///
    /// This function works with regular (C style) enumerations as well as enum classes.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The enumeration type, which derives from `CachedComponentData`, `ComponentType<Enum>`, and `CachedEnumData`.
    ///
    /// # Parameters
    ///
    /// - `enum_value`: The enumeration value.
    ///
    /// # Returns
    ///
    /// Returns the updated entity.
    pub fn add_enum_constant<T: CachedComponentData + ComponentType<Enum> + CachedEnumData>(
        self,
        enum_value: T,
    ) -> Self {
        let world = self.world;
        self.add_pair_from_ids(
            T::get_id(world),
            enum_value.get_entity_id_from_enum_field(world),
        )
    }

    pub fn add_pair_second<Second: CachedComponentData>(self, first: EntityT) -> Self {
        let world = self.world;
        self.add_pair_from_ids(first, Second::get_id(world))
    }

    pub fn add_component_id_if(self, component_id: IdT, condition: bool) -> Self {
        if condition {
            let world = self.world;
            return self.add_component_id(component_id);
        }

        self
    }

    pub fn add_component_if<T: CachedComponentData>(self, condition: bool) -> Self {
        let world = self.world;
        self.add_component_id_if(T::get_id(world), condition)
    }

    pub fn add_pair_ids_if(self, first: EntityT, mut second: EntityT, condition: bool) -> Self {
        let world = self.world;
        if condition {
            self.add_pair_from_ids(first, second)
        } else {
            // If second is 0 or if relationship is exclusive, use wildcard for
            // second which will remove all instances of the relationship.
            // Replacing 0 with Wildcard will make it possible to use the second
            // as the condition.
            if second == 0 || unsafe { ecs_has_id(self.world, first, EcsExclusive) } {
                second = unsafe { EcsWildcard }
            }
            self.remove_pair_from_ids(first, second)
        }
    }

    pub fn add_pair_if<T, U>(self, condition: bool) -> Self
    where
        T: CachedComponentData,
        U: CachedComponentData + ComponentType<Struct>,
    {
        let world = self.world;
        self.add_pair_ids_if(T::get_id(world), U::get_id(world), condition)
    }

    pub fn add_enum_tag_if<T, U>(self, enum_value: U, condition: bool) -> Self
    where
        T: CachedComponentData,
        U: CachedComponentData + ComponentType<Enum> + CachedEnumData,
    {
        let world = self.world;
        self.add_pair_ids_if(
            T::get_id(world),
            enum_value.get_entity_id_from_enum_field(world),
            condition,
        )
    }

    pub fn remove_component_id(self, component_id: IdT) -> Self {
        unsafe { ecs_remove_id(self.world, self.raw_id, component_id) }
        self
    }

    pub fn remove_component<T: CachedComponentData + ComponentType<Struct>>(self) -> Self {
        let world = self.world;
        self.remove_component_id(T::get_id(world))
    }

    /// Remove pair for enum
    /// This operation will remove any (Enum, *) pair from the entity.
    ///
    /// # Type parameters
    /// * `T` - The enum type.
    pub fn remove_component_enum<T: CachedComponentData + ComponentType<Enum>>(self) -> Self {
        let world = self.world;
        self.remove_pair_from_ids(T::get_id(world), unsafe { EcsWildcard })
    }

    pub fn remove_pair_from_ids(self, id: EntityT, id2: EntityT) -> Self {
        let world = self.world;
        self.remove_component_id(ecs_pair(id, id2))
    }

    pub fn remove_pair<T, U>(self) -> Self
    where
        T: CachedComponentData,
        U: CachedComponentData + ComponentType<Struct>,
    {
        let world = self.world;
        self.remove_pair_from_ids(T::get_id(world), U::get_id(world))
    }

    pub fn remove_enum_tag<T, U>(self, enum_value: U) -> Self
    where
        T: CachedComponentData,
        U: CachedComponentData + ComponentType<Enum> + CachedEnumData,
    {
        let world = self.world;
        self.remove_pair_from_ids(
            T::get_id(world),
            enum_value.get_entity_id_from_enum_field(world),
        )
    }

    pub fn remove_pair_second<Second: CachedComponentData>(self, first: EntityT) -> Self {
        let world = self.world;
        self.remove_pair_from_ids(first, Second::get_id(world))
    }

    //
    //
    //
    //
    //
    pub fn destruct(self) {
        unsafe { ecs_delete(self.world, self.raw_id) }
    }

    pub fn clear(&self) {
        unsafe { ecs_clear(self.world, self.raw_id) }
    }
}
