use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct Economy {
    pub money: u32,
    pub research_points: u32,
    pub materials: u32,
    pub energy: u32,
    
    // Passive generation rates per second
    pub money_generation: f32,
    pub research_generation: f32,
    pub energy_generation: f32,
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            money: 50,              // Reduced from 100 - players must be strategic
            research_points: 0,
            materials: 5,           // Reduced from 10 - materials should be scarce
            energy: 30,             // Reduced from 50 - energy management matters
            money_generation: 0.5,  // Drastically reduced from 2.0 - passive income less dominant
            research_generation: 0.3, // Reduced from 1.0 - research takes time
            energy_generation: 2.0,  // Reduced from 5.0 - energy scarcity
        }
    }
}

impl Economy {
    pub fn new(money: u32, research_points: u32, materials: u32, energy: u32) -> Self {
        Self {
            money,
            research_points,
            materials,
            energy,
            ..Default::default()
        }
    }

    pub fn can_afford(&self, cost: &ResourceCost) -> bool {
        self.money >= cost.money
            && self.research_points >= cost.research_points
            && self.materials >= cost.materials
            && self.energy >= cost.energy
    }

    pub fn spend(&mut self, cost: &ResourceCost) {
        if self.can_afford(cost) {
            self.money -= cost.money;
            self.research_points -= cost.research_points;
            self.materials -= cost.materials;
            self.energy -= cost.energy;
        }
    }

    pub fn earn(&mut self, reward: &ResourceReward) {
        self.money += reward.money;
        self.research_points += reward.research_points;
        self.materials += reward.materials;
        self.energy += reward.energy;
    }

    pub fn generate_passive_income(&mut self, delta_time: f32) {
        self.money += (self.money_generation * delta_time) as u32;
        self.research_points += (self.research_generation * delta_time) as u32;
        self.energy = (self.energy + (self.energy_generation * delta_time) as u32).min(100); // Cap energy at 100
    }

    pub fn get_total_value(&self) -> f32 {
        // Weighted value calculation for scoring/difficulty scaling
        self.money as f32 + 
        self.research_points as f32 * 2.0 + 
        self.materials as f32 * 3.0 + 
        self.energy as f32 * 0.5
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceCost {
    pub money: u32,
    pub research_points: u32,
    pub materials: u32,
    pub energy: u32,
}

impl ResourceCost {
    pub fn new(money: u32, research_points: u32, materials: u32, energy: u32) -> Self {
        Self {
            money,
            research_points,
            materials,
            energy,
        }
    }

    pub fn money(amount: u32) -> Self {
        Self {
            money: amount,
            research_points: 0,
            materials: 0,
            energy: 0,
        }
    }

    pub fn research(amount: u32) -> Self {
        Self {
            money: 0,
            research_points: amount,
            materials: 0,
            energy: 0,
        }
    }

    pub fn materials(amount: u32) -> Self {
        Self {
            money: 0,
            research_points: 0,
            materials: amount,
            energy: 0,
        }
    }

    pub fn energy(amount: u32) -> Self {
        Self {
            money: 0,
            research_points: 0,
            materials: 0,
            energy: amount,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceReward {
    pub money: u32,
    pub research_points: u32,
    pub materials: u32,
    pub energy: u32,
}

impl ResourceReward {
    pub fn new(money: u32, research_points: u32, materials: u32, energy: u32) -> Self {
        Self {
            money,
            research_points,
            materials,
            energy,
        }
    }

    pub fn money(amount: u32) -> Self {
        Self {
            money: amount,
            research_points: 0,
            materials: 0,
            energy: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TowerType {
    Basic,
    Advanced,
    Laser,
    Missile,
    Tesla,
}

impl TowerType {
    pub fn get_cost(&self) -> ResourceCost {
        match self {
            TowerType::Basic => ResourceCost::money(40),      // Increased from 25
            TowerType::Advanced => ResourceCost::new(80, 5, 3, 15),  // Increased costs
            TowerType::Laser => ResourceCost::new(120, 15, 2, 25),   // Increased costs
            TowerType::Missile => ResourceCost::new(160, 8, 6, 25),  // Increased costs
            TowerType::Tesla => ResourceCost::new(200, 20, 5, 40),   // Increased costs
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            TowerType::Basic => "Basic Tower",
            TowerType::Advanced => "Advanced Tower",
            TowerType::Laser => "Laser Tower",
            TowerType::Missile => "Missile Tower",
            TowerType::Tesla => "Tesla Tower",
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            TowerType::Basic => "Low cost, moderate damage",
            TowerType::Advanced => "Higher damage, requires materials",
            TowerType::Laser => "High accuracy, research required",
            TowerType::Missile => "Area damage, expensive materials",
            TowerType::Tesla => "Chain lightning, high energy cost",
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct TowerStats {
    pub tower_type: TowerType,
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32,
    pub last_shot: f32,
    pub upgrade_level: u32,
}

impl TowerStats {
    pub fn new(tower_type: TowerType) -> Self {
        let (damage, range, fire_rate) = match tower_type {
            TowerType::Basic => (12.0, 80.0, 0.8),     // Reduced damage and fire rate for balance
            TowerType::Advanced => (20.0, 100.0, 1.0),   // Reduced damage and fire rate
            TowerType::Laser => (15.0, 120.0, 1.8),      // Reduced damage and fire rate
            TowerType::Missile => (35.0, 90.0, 0.4),     // Reduced damage and fire rate
            TowerType::Tesla => (14.0, 70.0, 0.6),       // Reduced damage and fire rate
        };

        Self {
            tower_type,
            damage,
            range,
            fire_rate,
            last_shot: 0.0,
            upgrade_level: 1,
        }
    }

    pub fn can_shoot(&self, current_time: f32) -> bool {
        current_time - self.last_shot >= 1.0 / self.fire_rate
    }

    pub fn get_upgrade_cost(&self) -> ResourceCost {
        let base_cost = self.tower_type.get_cost();
        let multiplier = self.upgrade_level;
        
        ResourceCost::new(
            base_cost.money * multiplier / 2,
            base_cost.research_points * multiplier / 3,
            base_cost.materials * multiplier / 4,
            base_cost.energy * multiplier / 2,
        )
    }

    pub fn can_upgrade(&self) -> bool {
        self.upgrade_level < 5
    }

    pub fn upgrade(&mut self) {
        if !self.can_upgrade() {
            return;
        }

        self.upgrade_level += 1;
        self.apply_upgrade_stats();
    }

    fn apply_upgrade_stats(&mut self) {
        // Calculate base stats for level 1
        let (base_damage, base_range, base_fire_rate) = match self.tower_type {
            TowerType::Basic => (15.0, 80.0, 1.0),
            TowerType::Advanced => (25.0, 100.0, 1.2),
            TowerType::Laser => (20.0, 120.0, 2.0),
            TowerType::Missile => (40.0, 90.0, 0.5),
            TowerType::Tesla => (18.0, 70.0, 0.8),
        };

        // Apply level-based multipliers with tower-specific specializations
        let level_multiplier = self.upgrade_level as f32;
        
        match self.tower_type {
            TowerType::Basic => {
                // Balanced upgrade across all stats - REBALANCED for fair progression
                self.damage = base_damage * (1.0 + (level_multiplier - 1.0) * 0.15);     // Reduced from 0.25
                self.range = base_range * (1.0 + (level_multiplier - 1.0) * 0.12);       // Reduced from 0.15
                self.fire_rate = base_fire_rate * (1.0 + (level_multiplier - 1.0) * 0.15); // Reduced from 0.20
            },
            TowerType::Advanced => {
                // Focus on damage improvement - REBALANCED to prevent overpowering
                self.damage = base_damage * (1.0 + (level_multiplier - 1.0) * 0.18);     // Reduced from 0.35
                self.range = base_range * (1.0 + (level_multiplier - 1.0) * 0.10);       // Reduced from 0.12
                self.fire_rate = base_fire_rate * (1.0 + (level_multiplier - 1.0) * 0.12); // Reduced from 0.15
            },
            TowerType::Laser => {
                // Focus on fire rate (high accuracy, rapid fire) - REBALANCED
                self.damage = base_damage * (1.0 + (level_multiplier - 1.0) * 0.15);     // Consistent scaling
                self.range = base_range * (1.0 + (level_multiplier - 1.0) * 0.08);       // Reduced from 0.10
                self.fire_rate = base_fire_rate * (1.0 + (level_multiplier - 1.0) * 0.25); // Reduced from 0.40
            },
            TowerType::Missile => {
                // Focus on damage (area damage, explosive) - REBALANCED to prevent dominance
                self.damage = base_damage * (1.0 + (level_multiplier - 1.0) * 0.20);     // Reduced from 0.45
                self.range = base_range * (1.0 + (level_multiplier - 1.0) * 0.08);       // Reduced from 0.10
                self.fire_rate = base_fire_rate * (1.0 + (level_multiplier - 1.0) * 0.08); // Reduced from 0.10
            },
            TowerType::Tesla => {
                // Focus on range (chain lightning, area coverage) - REBALANCED
                self.damage = base_damage * (1.0 + (level_multiplier - 1.0) * 0.15);     // Reduced from 0.25
                self.range = base_range * (1.0 + (level_multiplier - 1.0) * 0.20);       // Reduced from 0.30
                self.fire_rate = base_fire_rate * (1.0 + (level_multiplier - 1.0) * 0.12); // Reduced from 0.15
            },
        }
    }
}

// Resource generation events
#[derive(Event)]
pub struct ResourceGeneratedEvent {
    pub reward: ResourceReward,
    pub source: String,
}

// Economy update events  
#[derive(Event)]
pub struct EconomyUpdateEvent {
    pub economy: Economy,
}