use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    // "level" generated using the Rusty Engine level creator example, and then tweaked
    let player = game
        .game_state_mut()
        .add_actor("player", ActorPreset::RacingCarYellow);
    player.translation = Vec2::new(-9.0, 23.0);
    player.rotation = DOWN;
    player.scale = 0.5;
    player.layer = 100.0;
    player.collision = true;

    let a = game
        .game_state_mut()
        .add_actor("82", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(131.7, -155.7);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.18000001;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny195", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-523.0, -323.4);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.66999966;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("88", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(333.6, -214.1);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.24000004;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("133", ActorPreset::RollingBlockNarrow);
    a.translation = Vec2::new(-524.9, -105.2);
    a.rotation = -21.99115372;
    a.scale = 0.62918293;
    a.layer = 0.52999979;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny205", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(193.8, -217.4);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.76999956;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("247", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-235.1, 3.2);
    a.rotation = -37.69910812;
    a.scale = 0.27740741;
    a.layer = 1.02999938;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny277", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-132.7, 294.8);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.26999915;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny217", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(314.6, 73.1);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.88999945;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny275", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-121.7, 67.1);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.24999917;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("97", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(389.6, -241.7);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.32999995;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("255", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-255.8, 265.2);
    a.rotation = -54.97783279;
    a.scale = 0.27740741;
    a.layer = 1.10999930;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("138", ActorPreset::RollingBallRed);
    a.translation = Vec2::new(103.4, 171.2);
    a.rotation = -21.99115372;
    a.scale = 0.42104703;
    a.layer = 0.55999976;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("253", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-190.8, 269.7);
    a.rotation = -51.05084991;
    a.scale = 0.27740741;
    a.layer = 1.08999932;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("84", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(202.5, -142.7);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.20000002;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("99", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(476.3, -243.5);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.34999993;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("62", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(48.4, -122.9);
    a.rotation = -14.13716888;
    a.scale = 0.23667733;
    a.layer = 0.06999999;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("92", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(188.6, -289.6);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.28000000;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("248", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-193.1, 20.5);
    a.rotation = -46.33847046;
    a.scale = 0.27740741;
    a.layer = 1.03999937;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("148", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(423.5, 342.5);
    a.rotation = -21.99115372;
    a.scale = 0.40432015;
    a.layer = 0.61999971;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("142", ActorPreset::RollingBallBlue);
    a.translation = Vec2::new(244.8, 317.8);
    a.rotation = -21.99115372;
    a.scale = 0.53833103;
    a.layer = 0.57999974;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("153", ActorPreset::RacingCarBlue);
    a.translation = Vec2::new(-602.6, -139.3);
    a.rotation = -21.99115372;
    a.scale = 0.50159109;
    a.layer = 0.62999970;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("153b", ActorPreset::RacingCarRed);
    a.translation = Vec2::new(330.0, -155.0);
    a.rotation = -1.0;
    a.scale = 0.50159109;
    a.layer = 0.62999970;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny213", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(580.8, -7.4);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.84999949;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("101", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(471.8, -190.8);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.36999992;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny266", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-454.4, 79.6);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.15999925;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny276", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-121.0, 182.4);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.25999916;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("238", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-514.0, 22.3);
    a.rotation = -22.77655220;
    a.scale = 0.27740741;
    a.layer = 0.93999940;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("131", ActorPreset::RollingBlockNarrow);
    a.translation = Vec2::new(-365.7, -105.9);
    a.rotation = -21.99115372;
    a.scale = 0.62918293;
    a.layer = 0.50999981;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("243", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-514.2, 268.3);
    a.rotation = -27.48894310;
    a.scale = 0.27740741;
    a.layer = 0.98999935;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("64", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(47.5, -334.8);
    a.rotation = -17.27876282;
    a.scale = 0.23667733;
    a.layer = 0.08999999;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("143", ActorPreset::RollingBallBlue);
    a.translation = Vec2::new(249.2, 181.4);
    a.rotation = -21.99115372;
    a.scale = 0.53833103;
    a.layer = 0.58999974;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny271", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-562.8, -11.2);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.20999920;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("67", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(179.0, -105.3);
    a.rotation = -18.84955978;
    a.scale = 0.23667733;
    a.layer = 0.11999998;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny206", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(292.7, -272.6);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.77999955;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("16", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(-75.9, 44.2);
    a.rotation = -1.57079637;
    a.scale = 0.50000000;
    a.layer = 0.02000000;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("100", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(493.8, -217.6);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.35999992;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("95", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(291.9, -346.9);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.30999997;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny203", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-386.2, -214.5);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.74999958;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny274", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-153.1, -40.5);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.23999918;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("66", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(129.5, -105.8);
    a.rotation = -18.84955978;
    a.scale = 0.23667733;
    a.layer = 0.10999998;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("96", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(353.3, -229.3);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.31999996;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny212", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(453.9, 90.6);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.83999950;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("70", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(328.3, -105.8);
    a.rotation = -18.84955978;
    a.scale = 0.23667733;
    a.layer = 0.14999999;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("244", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-471.1, 285.9);
    a.rotation = -28.27434158;
    a.scale = 0.27740741;
    a.layer = 0.99999934;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("63", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(48.4, -171.5);
    a.rotation = -14.13716888;
    a.scale = 0.23667733;
    a.layer = 0.07999999;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("123", ActorPreset::RollingBlockSquare);
    a.translation = Vec2::new(603.9, -344.4);
    a.rotation = -20.42035675;
    a.scale = 0.36985010;
    a.layer = 0.44999984;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny196", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-474.3, -321.8);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.67999965;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny216", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(319.8, 233.2);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.87999946;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("80", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(75.9, -186.7);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.16000000;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny209", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(492.3, -138.5);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.80999953;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("93", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(218.2, -309.5);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.28999999;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("115", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(505.8, -50.2);
    a.rotation = -19.63495827;
    a.scale = 0.45660508;
    a.layer = 0.42999986;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny269", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-578.8, 200.7);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.18999922;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny210", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(377.4, -26.8);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.81999952;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("90", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(109.8, -298.1);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.26000002;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("103", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(626.5, -152.6);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.38999990;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny197", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-314.0, -322.0);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.68999964;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("91", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(139.7, -288.1);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.27000001;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("235", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-351.4, 3.1);
    a.rotation = -21.99115372;
    a.scale = 0.28589886;
    a.layer = 0.90999943;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("68", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(229.1, -105.5);
    a.rotation = -18.84955978;
    a.scale = 0.23667733;
    a.layer = 0.12999998;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("237", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-471.7, 3.2);
    a.rotation = -21.99115372;
    a.scale = 0.27740741;
    a.layer = 0.92999941;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny268", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-574.0, 308.4);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.17999923;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("89", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(79.4, -313.9);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.25000003;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("18", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(-9.5, 81.7);
    a.rotation = -3.14159274;
    a.scale = 0.50000000;
    a.layer = 0.04000000;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("124", ActorPreset::RollingBlockSquare);
    a.translation = Vec2::new(-231.7, -318.9);
    a.rotation = -20.42035675;
    a.scale = 1.24493289;
    a.layer = 0.45999983;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny278", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-254.9, 335.7);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.27999914;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("86", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(265.5, -173.4);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.22000003;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("94", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(253.6, -333.6);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.29999998;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("130", ActorPreset::RollingBlockNarrow);
    a.translation = Vec2::new(-285.4, -105.8);
    a.rotation = -21.99115372;
    a.scale = 0.62918293;
    a.layer = 0.49999979;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny202", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-186.3, -209.4);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.73999959;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("245", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-447.9, 264.2);
    a.rotation = -29.84513855;
    a.scale = 0.27740741;
    a.layer = 1.00999939;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("164", ActorPreset::RacingCarGreen);
    a.translation = Vec2::new(-359.1, -139.2);
    a.rotation = -21.99115372;
    a.scale = 0.50159109;
    a.layer = 0.64999968;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny267", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-238.2, 73.4);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.16999924;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("250", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-174.0, 121.7);
    a.rotation = -51.83624649;
    a.scale = 0.27740741;
    a.layer = 1.05999935;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny270", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-582.6, 88.4);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.19999921;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny215", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(483.7, 283.9);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.86999947;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("87", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(297.6, -194.3);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.23000003;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("125", ActorPreset::RollingBlockSquare);
    a.translation = Vec2::new(-396.0, -319.2);
    a.rotation = -20.42035675;
    a.scale = 1.24493289;
    a.layer = 0.46999982;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny272", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-402.3, -62.7);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.21999919;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("28", ActorPreset::RollingBlockCorner);
    a.translation = Vec2::new(-115.6, -146.2);
    a.rotation = -9.42477798;
    a.scale = 0.79307318;
    a.layer = 0.05000000;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny263", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-209.9, 236.4);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.12999928;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("17", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(56.7, 44.9);
    a.rotation = -1.57079637;
    a.scale = 0.50000000;
    a.layer = 0.03000000;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("155", ActorPreset::RacingCarBlack);
    a.translation = Vec2::new(-525.8, -139.8);
    a.rotation = -21.99115372;
    a.scale = 0.50159109;
    a.layer = 0.63999969;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("105", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(556.9, -104.6);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.40999988;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("102", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(440.4, -184.4);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.37999991;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("242", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-530.8, 228.2);
    a.rotation = -26.70354462;
    a.scale = 0.27740741;
    a.layer = 0.97999936;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("69", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(278.8, -105.9);
    a.rotation = -18.84955978;
    a.scale = 0.23667733;
    a.layer = 0.13999999;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("114", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(393.1, -140.1);
    a.rotation = -19.63495827;
    a.scale = 0.45660508;
    a.layer = 0.41999987;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("98", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(433.6, -244.5);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.33999994;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("119", ActorPreset::RacingBarrelBlue);
    a.translation = Vec2::new(621.2, -319.8);
    a.rotation = -19.63495827;
    a.scale = 0.45660508;
    a.layer = 0.43999985;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("119b", ActorPreset::RacingBarrelRed);
    a.translation = Vec2::new(-235.0, -260.0);
    a.rotation = -17.63495827;
    a.scale = 0.4660508;
    a.layer = 0.43999985;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny211", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(199.8, -55.9);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.82999951;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny200", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-602.2, -188.1);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.71999961;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("81", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(99.5, -169.2);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.17000000;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("144", ActorPreset::RollingBallBlue);
    a.translation = Vec2::new(551.8, 96.8);
    a.rotation = -21.99115372;
    a.scale = 0.33928153;
    a.layer = 0.59999973;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny279", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-458.4, 333.4);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.28999913;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("48", ActorPreset::RollingBlockNarrow);
    a.translation = Vec2::new(-75.8, -231.7);
    a.rotation = -10.99557495;
    a.scale = 0.61090577;
    a.layer = 0.05999999;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("246", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-292.3, 3.2);
    a.rotation = -34.55752182;
    a.scale = 0.27740741;
    a.layer = 1.01999938;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("132", ActorPreset::RollingBlockNarrow);
    a.translation = Vec2::new(-445.4, -105.4);
    a.rotation = -21.99115372;
    a.scale = 0.62918293;
    a.layer = 0.51999980;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny198", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-12.1, -123.8);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.69999963;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("254", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-233.8, 287.1);
    a.rotation = -56.54862595;
    a.scale = 0.27740741;
    a.layer = 1.09999931;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("137", ActorPreset::RollingBallRed);
    a.translation = Vec2::new(207.0, 35.6);
    a.rotation = -21.99115372;
    a.scale = 0.74229646;
    a.layer = 0.54999977;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("146", ActorPreset::RollingBallBlueAlt);
    a.translation = Vec2::new(619.6, 341.4);
    a.rotation = -21.99115372;
    a.scale = 0.88000906;
    a.layer = 0.60999972;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny265", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-348.9, 116.4);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.14999926;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny201", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-78.5, -321.2);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.72999960;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny214", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(578.1, 220.8);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.85999948;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("252", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-173.5, 227.3);
    a.rotation = -51.83624649;
    a.scale = 0.27740741;
    a.layer = 1.07999933;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny218", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(68.9, 279.9);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.89999944;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny204", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(102.3, -235.8);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.75999957;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("126", ActorPreset::RollingBlockSquare);
    a.translation = Vec2::new(-600.7, -319.5);
    a.rotation = -20.42035675;
    a.scale = 1.24493289;
    a.layer = 0.47999981;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny194", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-442.6, -137.5);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.65999967;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("65", ActorPreset::RacingBarrierRed);
    a.translation = Vec2::new(79.9, -105.5);
    a.rotation = -18.84955978;
    a.scale = 0.23667733;
    a.layer = 0.09999999;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("134", ActorPreset::RollingBlockNarrow);
    a.translation = Vec2::new(-605.2, -105.3);
    a.rotation = -21.99115372;
    a.scale = 0.62918293;
    a.layer = 0.53999978;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("104", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(586.7, -125.8);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.39999989;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("239", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-531.5, 65.0);
    a.rotation = -23.56195068;
    a.scale = 0.27740741;
    a.layer = 0.94999939;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("241", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-530.8, 182.6);
    a.rotation = -23.56195068;
    a.scale = 0.27740741;
    a.layer = 0.96999937;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("83", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(167.3, -143.4);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.19000001;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny199", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-9.3, -235.8);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.70999962;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("240", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-531.0, 124.2);
    a.rotation = -23.56195068;
    a.scale = 0.27740741;
    a.layer = 0.95999938;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny207", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(446.1, -310.1);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.78999954;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny208", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(582.9, -256.5);
    a.rotation = -21.99115372;
    a.scale = 0.39820361;
    a.layer = 0.79999954;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny273", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-256.7, -64.1);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.22999918;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("140", ActorPreset::RollingBallRedAlt);
    a.translation = Vec2::new(455.0, 197.4);
    a.rotation = -21.99115372;
    a.scale = 0.74591058;
    a.layer = 0.56999975;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("249", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-174.3, 64.2);
    a.rotation = -51.83624649;
    a.scale = 0.27740741;
    a.layer = 1.04999936;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("129", ActorPreset::RollingBlockNarrow);
    a.translation = Vec2::new(-206.3, -105.4);
    a.rotation = -21.99115372;
    a.scale = 0.62918293;
    a.layer = 0.48999980;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("85", ActorPreset::RacingConeStraight);
    a.translation = Vec2::new(231.8, -154.0);
    a.rotation = -18.84955978;
    a.scale = 0.45660508;
    a.layer = 0.21000002;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny264", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-349.5, 265.3);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.13999927;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("236", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-411.3, 3.2);
    a.rotation = -21.99115372;
    a.scale = 0.28589886;
    a.layer = 0.91999942;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("251", ActorPreset::RacingBarrierWhite);
    a.translation = Vec2::new(-173.5, 179.8);
    a.rotation = -51.83624649;
    a.scale = 0.27740741;
    a.layer = 1.06999934;
    a.collision = true;
    let a = game
        .game_state_mut()
        .add_actor("shiny262", ActorPreset::RollingHoleStart);
    a.translation = Vec2::new(-487.5, 229.7);
    a.rotation = -54.97783279;
    a.scale = 0.39806983;
    a.layer = 1.11999929;
    a.collision = true;

    // Music!
    game.game_state_mut()
        .audio_manager
        .play_music(MusicPreset::Classy8Bit, 0.5);

    // Stuff used to keep and display score
    game.game_state_mut().u32_map.insert("score".into(), 0); // actual score
    let score_text = game.add_text_actor("score_text", "Score: 0");
    score_text.translation = Vec2::new(-10.0, 82.0);
    score_text.font_size = 24.0;

    // Win condition
    let win_amount = game
        .game_state_mut()
        .actors
        .values()
        .filter(|a| a.preset == Some(ActorPreset::RollingHoleStart))
        .count() as u32;
    game.game_state_mut()
        .u32_map
        .insert("win_amount".into(), win_amount);
    // stateful boolean indicating whether we won
    game.game_state_mut().bool_vec.push(false);

    // Crashed
    game.game_state_mut()
        .bool_map
        .insert("crashed".into(), false);

    // Velocity of player
    game.game_state_mut()
        .vec2_map
        .insert("velocity".into(), Vec2::ZERO);

    game.run(logic);
}

const TURN_RATE: f32 = 3.0;
const ACCELERATION_RATE: f32 = 100.0;

fn logic(game_state: &mut GameState) {
    let mut win = game_state.bool_vec.get(0).unwrap().clone();
    let crashed = game_state.bool_map.get("crashed").unwrap().clone();
    let win_amount = game_state.u32_map.get_mut("win_amount").unwrap().clone();
    let score = game_state.u32_map.get_mut("score").unwrap();
    let score_text = game_state.text_actors.get_mut("score_text").unwrap();
    let velocity = game_state.vec2_map.get_mut("velocity").unwrap();

    // if *win {
    //     for actor in game_state.actors.values_mut() {
    //         if actor.label == "player" {
    //             continue;
    //         }
    //         //actor.translation *= 1.0 + 1.5 * game_state.delta_f32;
    //         actor.rotation += 1.0 * game_state.delta_f32;
    //     }
    //     return;
    // }

    if crashed {
        return;
    }

    // Player movement
    let player = game_state.actors.get_mut("player".into()).unwrap();
    let mut acceleration = 0.0;
    let mut rotation = 0.0;
    // Nested scope so the bare KeyCode variants only show up here where we want to use them
    {
        use KeyCode::*;
        // Acceleration input
        if game_state.keyboard_state.pressed_any(&[W, Up, Comma]) {
            acceleration += 1.0;
        }
        if game_state.keyboard_state.pressed_any(&[S, Down, O]) {
            acceleration -= 1.0;
        }
        // Rotation/Turning input
        if game_state.keyboard_state.pressed_any(&[A, Left]) {
            rotation += 1.0;
        }
        if game_state.keyboard_state.pressed_any(&[D, Right, E]) {
            rotation -= 1.0;
        }
    }
    let mut velocity_magnitude = velocity.length();
    velocity_magnitude += (acceleration * ACCELERATION_RATE) * game_state.delta_f32;
    player.rotation += (rotation * TURN_RATE) * game_state.delta_f32;
    *velocity = Vec2::new(
        velocity_magnitude * player.rotation.cos(),
        velocity_magnitude * player.rotation.sin(),
    );
    player.translation += *velocity * game_state.delta_f32;

    // Make the shinies...shinier
    for actor in game_state
        .actors
        .values_mut()
        .filter(|a| a.label.starts_with("shiny"))
    {
        actor.scale = 0.25 + 0.03 * ((game_state.time_since_startup_f64 * 6.0).cos() as f32);
    }

    // Don't do stuff past this point after we win
    if win {
        return;
    }

    // Process collisions
    for event in game_state.collision_events.drain(..) {
        if !event.pair.either_contains("player") {
            // if it doesn't involve the player, we don't care
            continue;
        }
        if event.state.is_end() {
            // we don't care about the player _ending_ a collision with anything
            continue;
        }
        // Collect shinies!
        if event.pair.one_starts_with("shiny") {
            let shiny_label = if event.pair.0.starts_with("shiny") {
                event.pair.0.clone()
            } else {
                event.pair.1.clone()
            };
            game_state.actors.remove(&shiny_label);
            game_state
                .audio_manager
                .play_sfx(SfxPreset::Confirmation1, 1.0);
            *score += 1;
            score_text.text = format!("Score: {}", score);
            if *score >= win_amount {
                win = true;
                *game_state.bool_vec.get_mut(0).unwrap() = true;
            }
            continue;
        }

        // Crash!
        *game_state.bool_map.get_mut("crashed").unwrap() = true;
        //game_state.add_text_actor("crashed", "You crashed. You fail. :-(");
        game_state.audio_manager.play_sfx(SfxPreset::Jingle3, 1.0);
        game_state.audio_manager.stop_music();
    }

    if win {
        game_state
            .audio_manager
            .play_sfx(SfxPreset::Congratulations, 1.0);
        let mut you_win = game_state.add_text_actor("you win", "You Win!");
        you_win.font_size = 120.0;
        you_win.translation.y = -50.0;
    }
}
