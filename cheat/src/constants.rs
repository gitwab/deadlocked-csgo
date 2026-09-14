pub mod cs2 {
    use shared::Weapon;

    pub const PROCESS_NAME: &str = "csgo_linux64";
    pub const CLIENT_LIB: &str = "client_client.so";
    pub const ENGINE_LIB: &str = "engine_client.so";
    pub const TIER0_LIB: &str = "libtier0_client.so";
    pub const INPUT_LIB: &str = "inputsystem_client.so";
    pub const SDL_LIB: &str = "libSDL2.so.0";
    pub const SCHEMA_LIB: &str = "schemasystem_client.so";
    pub const PHYSICS_LIB: &str = "vphysics_client.so";

    pub const LIBS: [&str; 6] = [
        CLIENT_LIB, ENGINE_LIB, TIER0_LIB, INPUT_LIB, SDL_LIB, SCHEMA_LIB,
    ];

    pub const DEFAULT_FOV: u32 = 90;

    pub const MESH_SKELETON_BONE_COUNT: usize = 96;

    pub const SOUND_ESP_FOOTSTEP_DIAMETER_DEFAULT: f32 = 2000.0;
    pub const SOUND_ESP_GUNSHOT_DIAMETER_DEFAULT: f32 = 3000.0;
    pub const SOUND_ESP_WEAPON_DIAMETER_DEFAULT: f32 = 1000.0;

    pub const GRENADES: &[Weapon] = &[
        Weapon::Decoy,
        Weapon::Flashbang,
        Weapon::HE,
        Weapon::Incendiary,
        Weapon::Molotov,
        Weapon::Smoke,
    ];

    pub mod class {
        pub const PLAYER_CONTROLLER: &str = "CCSPlayer";

        pub const PLANTED_C4: &str = "CPlantedC4";
        pub const INFERNO: &str = "CInferno";
        pub const SMOKE: &str = "CSmokeGrenadeProjectile";
        pub const MOLOTOV: &str = "CMolotovProjectile";
        pub const FLASHBANG: &str = "CFlashbangProjectile";
        pub const HE_GRENADE: &str = "CHEGrenadeProjectile";
        pub const DECOY: &str = "CDecoyProjectile";

        pub const CHICKEN: &str = "CChicken";
    }
}

pub mod elf {
    pub const PROGRAM_HEADER_OFFSET: usize = 0x20;
    pub const PROGRAM_HEADER_ENTRY_SIZE: usize = 0x36;
    pub const PROGRAM_HEADER_NUM_ENTRIES: usize = 0x38;

    pub const SECTION_HEADER_OFFSET: usize = 0x28;
    pub const SECTION_HEADER_ENTRY_SIZE: usize = 0x3A;
    pub const SECTION_HEADER_NUM_ENTRIES: usize = 0x3C;

    pub const DYNAMIC_SECTION_PHT_TYPE: usize = 0x02;
}

pub const GRENADE_FILE_NAME: &str = "grenades.json";
