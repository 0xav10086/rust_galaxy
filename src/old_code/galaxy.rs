/// 表示一个天体的通用结构
#[derive(Debug, Clone)]
pub struct CelestialBody {
    pub name: String,
    pub mass: f64,          // 质量 (单位: kg)
    pub radius: f64,        // 半径 (单位: m)
    pub position: [f64; 3], // 位置 (单位: m)
    pub velocity: [f64; 3], // 速度 (单位: m/s)
    pub color: [f64; 3], //颜色
}

impl CelestialBody {
    /// 创建一个新的天体
    pub fn new(name: &str, mass: f64, radius: f64, position: [f64; 3], velocity: [f64; 3], color: [f64; 3]) -> Self {
        Self {
            name: name.to_string(),
            mass,
            radius,
            position,
            velocity,
            color,
        }
    }
}
/// 表示一个太阳系
#[derive(Debug)]
pub struct SolarSystem {
    pub star: CelestialBody,       // 恒星 (例如太阳)
    pub planets: Vec<CelestialBody>, // 行星列表
    pub satellites: Vec<CelestialBody>, // 卫星列表
}

impl SolarSystem {
    /// 创建一个新的太阳系
    pub fn new(star: CelestialBody) -> Self {
        Self {
            star,
            planets: Vec::new(),
            satellites: Vec::new(),
        }
    }

    /// 添加行星
    pub fn add_planet(&mut self, planet: CelestialBody) {
        self.planets.push(planet);
    }

    /// 添加卫星
    pub fn add_satellite(&mut self, satellite: CelestialBody) {
        self.satellites.push(satellite);
    }
}

pub fn create_solar_system() -> SolarSystem {
    let sun = CelestialBody::new(
        "Sun",
        1.989e30,          // 质量 (kg)
        6.9634e8,          // 半径 (m)
        [0.0, 0.0, 0.0],   // 位置 (m)
        [0.0, 0.0, 0.0],   // 速度 (m/s)
        [1.0, 1.0, 0.0],   // 颜色 (RGB)
    );

    let mercury = CelestialBody::new(
        "Mercury",
        3.285e23,          // 质量 (kg)
        2.4397e6,          // 半径 (m)
        [5.791e10, 0.0, 0.0], // 位置 (m)
        [0.0, 4.74e6, 0.0],   // 速度 (m/s)
        [0.8, 0.8, 0.0],   // 颜色 (RGB)
    );

    let venus = CelestialBody::new(
        "Venus",
        4.867e24,          // 质量 (kg)
        6.0518e6,          // 半径 (m)
        [1.082e11, 0.0, 0.0], // 位置 (m)
        [0.0, 3.502e4, 0.0],  // 速度 (m/s)
        [1.0, 0.5, 0.0],   // 颜色 (RGB)
    );

    let mars = CelestialBody::new(
        "Mars",
        6.4171e23,         // 质量 (kg)
        3.3895e6,          // 半径 (m)
        [2.279e11, 0.0, 0.0], // 位置 (m)
        [0.0, 2.407e4, 0.0],  // 速度 (m/s)
        [1.0, 0.0, 0.0],   // 颜色 (RGB)
    );

    let jupiter = CelestialBody::new(
        "Jupiter",
        1.898e27,          // 质量 (kg)
        6.9911e7,          // 半径 (m)
        [7.785e11, 0.0, 0.0], // 位置 (m)
        [0.0, 1.307e4, 0.0],  // 速度 (m/s)
        [0.0, 0.0, 1.0],   // 颜色 (RGB)
    );

    let saturn = CelestialBody::new(
        "Saturn",
        5.683e26,          // 质量 (kg)
        5.8232e7,          // 半径 (m)
        [1.429e12, 0.0, 0.0], // 位置 (m)
        [0.0, 9.69e3, 0.0],   // 速度 (m/s)
        [1.0, 1.0, 0.5],   // 颜色 (RGB)
    );

    let uranus = CelestialBody::new(
        "Uranus",
        8.681e25,          // 质量 (kg)
        2.5362e7,          // 半径 (m)
        [2.871e12, 0.0, 0.0], // 位置 (m)
        [0.0, 6.81e3, 0.0],   // 速度 (m/s)
        [0.5, 0.5, 1.0],   // 颜色 (RGB)
    );

    let neptune = CelestialBody::new(
        "Neptune",
        1.024e26,          // 质量 (kg)
        2.4622e7,          // 半径 (m)
        [4.495e12, 0.0, 0.0], // 位置 (m)
        [0.0, 5.43e3, 0.0],   // 速度 (m/s)
        [0.0, 0.5, 1.0],   // 颜色 (RGB)
    );  

    // 创建地球
    let earth = CelestialBody::new(
        "Earth",
        5.972e24,          // 质量 (kg)
        6.371e6,           // 半径 (m)
        [1.496e11, 0.0, 0.0], // 位置 (m)
        [0.0, 2.978e4, 0.0],  // 速度 (m/s)
        [0.0, 1.0, 0.0],   // 颜色 (RGB)
    );

    // 创建月球
    let moon = CelestialBody::new(
        "Moon",
        7.34767309e22,     // 质量 (kg)
        1.7374e6,          // 半径 (m)
        [1.496e11 + 3.844e8, 0.0, 0.0], // 位置 (m)
        [0.0, 2.978e4 + 1.022e3, 0.0],  // 速度 (m/s)
        [0.5, 0.5, 0.5],   // 颜色 (RGB)
    );

    let mut solar_system = SolarSystem::new(sun);
    solar_system.add_planet(mercury);   
    solar_system.add_planet(venus); 
    solar_system.add_planet(earth);
    solar_system.add_planet(mars);
    solar_system.add_planet(jupiter);
    solar_system.add_planet(saturn);
    solar_system.add_planet(uranus);
    solar_system.add_planet(neptune);
    solar_system.add_satellite(moon); 

    solar_system
}