# 🧟 Zombie Defense Game

<img src="https://shields.io" alt="Rust"> <img src="https://shields.io" alt="Bevy">

A 2D Tower Defense action game built with the **Bevy** engine in Rust. Defend your command center against endless waves of incoming zombies, earn gold, and survive as the threat level rises.

---

## 🎮 Gameplay

* **Base Defense:** Protect the main command center (`Base`). A protective `Wall` stands in front of the base to hold back enemies and automatically regenerates after being destroyed.
* **Shooting Mechanics:** Fire projectiles toward enemies by clicking the Left Mouse Button (LMB).
* **Progression System:** Defeating zombies awards gold. Once the kill quota is met and enough funds are available, the `ThreatLevel` automatically increases, raising the difficulty and enemy spawn limits.

---

## 👾 Zombie Types

Each zombie type possesses unique attributes and behaviors:
* **Normal Zombie:** A classic, slow-moving melee enemy.
* **Toxic Zombie:** A fast-moving variant capable of launching ranged projectiles at the wall.
* **Fat Zombie:** A high-health "tank" that moves slowly but deals devastating damage upon impact.

---

## 🛠️ Project Architecture

The codebase is organized into independent ECS modules:
* `base` — Command center logic, health management, and destruction.
* `player` — Character spawning and bullet shooting mechanics.
* `zombie` — Enemy AI, unique types, wave spawning, and combat behavior.
* `wall` — Defensive barrier mechanics, projectile collision, and respawn handling.
* `component` — Centralized storage for all components and resources (HP, Damage, Speed, Hitbox).
* `util` — AABB collision detection, entity movement systems, and level-up progression logic.

---

## 📝 Roadmap

- [ ] Connect the main menu system (`PlayButton`, `SettingsButton`, `QuitButton`)
- [ ] Integrate full graphical assets and custom sprites for all entities
- [ ] Implement an upgrade shop to boost weapon damage and wall durability using gold
