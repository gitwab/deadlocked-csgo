use crate::{
    constants::cs2,
    cs2::{CS2, schema::Schema},
};

macro_rules! field_stmt {
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, lib ($lib:ident)) => {{
        $off.$field.$fdef = $proc.module_base_address(cs2::$lib)?;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, interface ($lib:ident, $name:literal)) => {{
        let Some($fdef) = $proc.get_interface_offset($off.library.$lib, $name) else {
            utils::warn!(concat!(
                "could not find '",
                stringify!($fdef),
                "' interface offset"
            ));
            return None;
        };
        $off.$field.$fdef = $fdef;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, convar ($name:literal)) => {{
        let Some($fdef) = $proc.get_convar($off.interface.cvar, $name) else {
            utils::warn!(concat!(
                "could not find '",
                stringify!($fdef),
                "' convar offset"
            ));
            return None;
        };
        $off.$field.$fdef = $fdef;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, scan ($sig:literal, $lib:ident, rel($a:literal, $b:literal))) => {{
        let Some($fdef) = $proc.scan($sig, $off.library.$lib) else {
            utils::warn!(concat!("could not find '", stringify!($fdef), "' offset"));
            return None;
        };
        $off.$field.$fdef = $proc.get_relative_address($fdef, $a, $b);
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, scan ($sig:literal, $lib:ident, rel($a:literal, $b:literal, $c:literal))) => {{
        let Some($fdef) = $proc.scan($sig, $off.library.$lib) else {
            utils::warn!(concat!("could not find '", stringify!($fdef), "' offset"));
            return None;
        };
        $off.$field.$fdef = $proc.get_relative_address($fdef + $a, $b, $c);
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, scan_ptr ($sig:literal, $lib:ident, rel($a:literal, $b:literal))) => {{
        let Some($fdef) = $proc.scan($sig, $off.library.$lib) else {
            utils::warn!(concat!("could not find '", stringify!($fdef), "' offset"));
            return None;
        };
        let $fdef = $proc.get_relative_address($fdef, $a, $b);
        $off.$field.$fdef = $proc.read($fdef);
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, class_size ($class:literal)) => {{
        $off.$field.$fdef = $client.get_class($class)?.size() as _;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, fixed ($value:literal)) => {{
        $off.$field.$fdef = $value;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, schema ($class:literal, $field_name:literal)) => {{
        $off.$field.$fdef = $client.get($class, $field_name)?;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, phys ($class:literal, $field_name:literal)) => {{
        $off.$field.$fdef = $physics.get($class, $field_name)?;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, schema_sub ($class:literal, $field_name:literal, $sub:literal)) => {{
        $off.$field.$fdef = $client.get($class, $field_name)? - $sub;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, deref ($src:ident, $off_lit:literal, $add_lit:literal)) => {{
        $off.$field.$fdef = $proc.read::<usize>($off.$field.$src + $off_lit) + $add_lit;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, interface_fn ($grp:ident, $src:ident, $idx:literal, $off_lit:literal)) => {{
        $off.$field.$fdef = $proc
            .read::<u32>($proc.get_interface_function($off.$grp.$src, $idx) + $off_lit)
            as usize;
    }};
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, module_export ($lib:ident, $name:literal, rel($a:literal, $b:literal, $c:literal, $d:literal))) => {{
        let Some(window) = $proc.get_module_export($off.library.$lib, $name) else {
            utils::warn!(concat!("could not find '", stringify!($fdef), "' offset"));
            return None;
        };
        let window = $proc.get_relative_address(window, $a, $b);
        let window = $proc.read(window);
        $off.$field.$fdef = $proc.get_relative_address(window, $c, $d);
    }};
    // CS:GO specific offsets (fixed values)
    ($proc:ident, $off:ident, $client:ident, $physics:ident, $field:ident, $fdef:ident, csgo_fixed ($value:literal)) => {{
        $off.$field.$fdef = $value;
    }};
}

macro_rules! schema {
    (@struct $field:ident : $name:ident { $($def:tt)* } $($rest:tt)*) => {
        schema!(@sdef $name $($def)*);
        schema!(@struct $($rest)*);
    };
    (@struct $scope:ident : { $($inner:tt)* } $($rest:tt)*) => {
        schema!(@struct $($inner)*);
        schema!(@struct $($rest)*);
    };
    (@struct) => {};

    (@sdef $name:ident $($fdef:ident : $kind:ident $args:tt ,)*) => {
        #[derive(Default)]
        pub struct $name {
            $(pub $fdef: usize,)*
        }
    };

    (@offsets_acc { $($fields:tt)* } $field:ident : $name:ident { $($def:tt)* } $($rest:tt)*) => {
        schema!(@offsets_acc { $($fields)* pub $field: $name, } $($rest)*);
    };
    (@offsets_acc { $($fields:tt)* } $scope:ident : { $($inner:tt)* } $($rest:tt)*) => {
        schema!(@offsets_acc { $($fields)* } $($inner)* $($rest)*);
    };
    (@offsets_acc { $($fields:tt)* }) => {
        #[derive(Default)]
        pub struct Offsets {
            $($fields)*
        }
    };

    (@fn $proc:ident $off:ident $schema:ident $client:ident $physics:ident $field:ident : $name:ident { $($fdef:ident : $kind:ident $args:tt ,)* } $($rest:tt)+) => {
        $(field_stmt!($proc, $off, $client, $physics, $field, $fdef, $kind $args);)*
        schema!(@fn $proc $off $schema $client $physics $($rest)*);
    };
    (@fn $proc:ident $off:ident $schema:ident $client:ident $physics:ident $field:ident : $name:ident { $($fdef:ident : $kind:ident $args:tt ,)* }) => {
        $(field_stmt!($proc, $off, $client, $physics, $field, $fdef, $kind $args);)*
    };
    (@fn $proc:ident $off:ident $schema:ident $client:ident $physics:ident client: { $($inner:tt)* } $($rest:tt)+) => {
        let $schema = Schema::new($proc, $off.library.schema)?;
        let $client = $schema.get_library(cs2::CLIENT_LIB)?;
        schema!(@fn $proc $off $schema $client $physics $($inner)* $($rest)*);
    };
    (@fn $proc:ident $off:ident $schema:ident $client:ident $physics:ident client: { $($inner:tt)* }) => {
        let $schema = Schema::new($proc, $off.library.schema)?;
        let $client = $schema.get_library(cs2::CLIENT_LIB)?;
        schema!(@fn $proc $off $schema $client $physics $($inner)*)
    };
    (@fn $proc:ident $off:ident $schema:ident $client:ident $physics:ident physics: { $($inner:tt)* } $($rest:tt)+) => {
        let $physics = $schema.get_library(cs2::PHYSICS_LIB)?;
        schema!(@fn $proc $off $schema $client $physics $($inner)* $($rest)*);
    };
    (@fn $proc:ident $off:ident $schema:ident $client:ident $physics:ident physics: { $($inner:tt)* }) => {
        let $physics = $schema.get_library(cs2::PHYSICS_LIB)?;
        schema!(@fn $proc $off $schema $client $physics $($inner)*)
    };
    (@fn $proc:ident $off:ident $schema:ident $client:ident $physics:ident) => {};

    ($($group:tt)*) => {
        schema!(@struct $($group)*);
        schema!(@offsets_acc {} $($group)*);
        macro_rules! find_offsets_body {
            ($proc:ident, $off:ident, $schema:ident, $client:ident, $physics:ident) => {
                schema!(@fn $proc $off $schema $client $physics $($group)*)
            };
        }
    };
}

schema! {
    library: LibraryOffsets {
        client: lib(CLIENT_LIB),
        engine: lib(ENGINE_LIB),
        tier0: lib(TIER0_LIB),
        input: lib(INPUT_LIB),
        sdl: lib(SDL_LIB),
        schema: lib(SCHEMA_LIB),
        physics: lib(PHYSICS_LIB),
    }

    interface: InterfaceOffsets {
        resource: interface(engine, "GameResourceServiceClientV0"),
        entity: deref(resource, 0x50, 0x10),
        cvar: interface(tier0, "VEngineCvar0"),
        input: interface(input, "InputSystemVersion0"),
    }

    direct: DirectOffsets {
        // CS:GO offsets - using fixed offsets provided
        local_player: csgo_fixed(0xDEA98C),
        entity_list: csgo_fixed(0x4DDC8F4),
        view_matrix: csgo_fixed(0x4D0F250),
        button_state: interface_fn(interface, input, 19, 0x14),
        sdl_window: module_export(sdl, "SDL_GetKeyboardFocus", rel(0x02, 0x06, 0x03, 0x07)),
        global_vars: scan("48 8D 05 ? ? ? ? 45 31 E4 48 8B 00 8B 78 10", engine, rel(0x03, 0x07)),
        vphys_world: scan_ptr("4c 8d 35 ? ? ? ? 49 8b 3e e8 ? ? ? ? 48 89 c2", engine, rel(3, 7)),
        build_date: scan("4c 89 e6 e8 ? ? ? ? 48 8d 35 ? ? ? ? 48 8d 3d", engine, rel(11, 15)),
    }

    convar: ConvarOffsets {
        ffa: convar("mp_teammates_are_enemies"),
        sensitivity: convar("sensitivity"),
    }

    client: {
        controller: PlayerControllerOffsets {
            // CS:GO entity offsets
            steam_id: csgo_fixed(0x1E8),
            money_services: csgo_fixed(0x1F0),
            money: csgo_fixed(0x1F0),
            color: csgo_fixed(0x204),
            name: csgo_fixed(0x210),
            pawn: csgo_fixed(0x1C8),
            desired_fov: csgo_fixed(0x238),
            owner_entity: csgo_fixed(0xF4),
            rank: csgo_fixed(0x1324),
            rank_type: csgo_fixed(0x1328),
            action_tracking_services: csgo_fixed(0x1288),
        }
        entity: EntityOffsets {
            // CS:GO player entity offsets
            health: csgo_fixed(0x100),
            max_health: csgo_fixed(0x108),
            team: csgo_fixed(0xF4),
            life_state: csgo_fixed(0x25F),
            game_scene_node: csgo_fixed(0x318),
            velocity: csgo_fixed(0x114),
            collision: csgo_fixed(0x310),
        }
        collision: CollisionOffsets {
            mins: csgo_fixed(0x8),
            maxs: csgo_fixed(0x14),
        }
        pawn: PawnOffsets {
            controller: csgo_fixed(0x1A0),
            armor: csgo_fixed(0x104),
            fov_multiplier: csgo_fixed(0x110),
            eye_offset: csgo_fixed(0x10C),
            shots_fired: csgo_fixed(0x29C),
            view_angles: csgo_fixed(0x288),
            eye_angles: csgo_fixed(0x290),
            flags: csgo_fixed(0x100),
            crosshair_entity: csgo_fixed(0x2A8),
            is_scoped: csgo_fixed(0x2A0),
            deathmatch_immunity: csgo_fixed(0x2A4),
            observer_services: csgo_fixed(0x340),
            spotted_state: csgo_fixed(0x350),
            flash_alpha: csgo_fixed(0x20C),
            flash_duration: csgo_fixed(0x210),
            camera_services: csgo_fixed(0x330),
            item_services: csgo_fixed(0x328),
            weapon_services: csgo_fixed(0x320),
            aim_punch_services: csgo_fixed(0x360),
            bullet_services: csgo_fixed(0x370),
        }
        game_scene_node: GameSceneNodeOffsets {
            dormant: csgo_fixed(0xED),
            origin: csgo_fixed(0x138),
            node_to_world: csgo_fixed(0x30),
            model_state: csgo_fixed(0x168),
            model_name: csgo_fixed(0x8),
        }
        smoke: SmokeOffsets {
            did_smoke_effect: csgo_fixed(0x3E8),
            smoke_color: csgo_fixed(0x3F0),
        }
        molotov: MolotovOffsets {
            is_incendiary: csgo_fixed(0x3E8),
        }
        inferno: InfernoOffsets {
            is_burning: csgo_fixed(0x3E8),
            fire_count: csgo_fixed(0x3F0),
            fire_positions: csgo_fixed(0x3F8),
        }
        spotted_state: SpottedStateOffsets {
            spotted: csgo_fixed(0x0),
            mask: csgo_fixed(0x1),
        }
        camera_services: CameraServicesOffsets {
            fov: csgo_fixed(0x0),
        }
        item_services: ItemServicesOffsets {
            has_defuser: csgo_fixed(0x0),
            has_helmet: csgo_fixed(0x4),
        }
        weapon_services: WeaponServicesOffsets {
            weapons: csgo_fixed(0x0),
            active_weapon: csgo_fixed(0x4),
        }
        aim_punch_services: AimPunchServicesOffsets {
            aim_punch_cache: csgo_fixed(0x0),
        }
        action_tracking: ActionTrackingServicesOffsets {
            round_kills: csgo_fixed(0x0),
            round_damage: csgo_fixed(0x4),
            per_round_stats: csgo_fixed(0x8),
        }
        bullet_services: BulletServicesOffsets {
            total_hits: csgo_fixed(0x0),
        }
        per_round_stats: PerRoundStatsOffsets {
            kills: csgo_fixed(0x0),
            deaths: csgo_fixed(0x4),
            assists: csgo_fixed(0x8),
            damage: csgo_fixed(0xC),
            size: csgo_fixed(0x20),
        }
        observer_services: ObserverServicesOffsets {
            target: csgo_fixed(0x0),
        }
        econ_item_view: EconItemViewOffsets {
            item_definition_index: csgo_fixed(0x0),
        }
        weapon: WeaponOffsets {
            attribute_manager: csgo_fixed(0x0),
            item: csgo_fixed(0x0),
            item_definition_index: csgo_fixed(0x0),
            clip_primary: csgo_fixed(0x0),
            reserve_ammo: csgo_fixed(0x0),
        }
        planted_c4: PlantedC4Offsets {
            is_ticking: csgo_fixed(0x3E8),
            blow_time: csgo_fixed(0x3F0),
            being_defused: csgo_fixed(0x3F8),
            is_defused: csgo_fixed(0x3FC),
            has_exploded: csgo_fixed(0x400),
            defuse_time: csgo_fixed(0x404),
            defuse_time_left: csgo_fixed(0x408),
        }
        model_state: ModelState {
            skeleton_instance: csgo_fixed(0x0),
        }
        network_velocity: NetworkVelocityOffsets {
            x: fixed(0x10),
            y: fixed(0x18),
            z: fixed(0x20),
        }
        entity_identity: EntityIdentityOffsets {
            size: csgo_fixed(0x50),
        }
    }

    physics: {
        hull: PhysHullOffsets {
            vertices: csgo_fixed(0x0),
            edges: csgo_fixed(0x8),
            faces: csgo_fixed(0x10),
            flags: csgo_fixed(0x18),
        }
        mesh: PhysMeshOffsets {
            vertices: csgo_fixed(0x0),
            triangles: csgo_fixed(0x8),
            materials: csgo_fixed(0x10),
            flags: csgo_fixed(0x18),
        }
    }
}

impl CS2 {
    pub fn find_offsets(&self) -> Option<Offsets> {
        let proc = &self.process;
        let mut offsets = Offsets::default();
        find_offsets_body!(proc, offsets, schema, client, physics);
        Some(offsets)
    }
}
