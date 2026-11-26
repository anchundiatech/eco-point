#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env,
};

const POINTS_PER_KG: u32 = 10;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EcoError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidAmount = 3,
    Overflow = 4,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Points(Address),
    TotalKg(Address),
    DeliveryCount(Address),
    Delivery(Address, u32),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delivery {
    pub kilograms: u32,
    pub points: u32,
}

#[contractevent(topics = ["delivery"], data_format = "vec")]
pub struct DeliveryRecorded {
    #[topic]
    beneficiary: Address,
    kilograms: u32,
    points: u32,
}

#[contract]
pub struct EcoPuntos;

#[contractimpl]
impl EcoPuntos {
     // Inicializamo el contrato con la direccion autorizada para registrar entregas y otorgar puntos
    pub fn initialize(env: Env, admin: Address) -> Result<(), EcoError> {
        let storage = env.storage().instance();
        if storage.has(&DataKey::Admin) {
            return Err(EcoError::AlreadyInitialized);
        }

        admin.require_auth();
        storage.set(&DataKey::Admin, &admin);
        Ok(())
    }

    // Función para registrar una entrega de residuos y otorgar puntos al usuario
    // Solo el administrador puede registrar las entregas
    // 1 kg de residuos reciclables otorga 10 puntos 
    pub fn record_delivery(
        env: Env,
        beneficiary: Address,
        kilograms: u32,
    ) -> Result<u32, EcoError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(EcoError::NotInitialized)?;

        admin.require_auth();

        if kilograms == 0 {
            return Err(EcoError::InvalidAmount);
        }

        let points = kilograms
            .checked_mul(POINTS_PER_KG)
            .ok_or(EcoError::Overflow)?;

        let points_key = DataKey::Points(beneficiary.clone());
        let kg_key = DataKey::TotalKg(beneficiary.clone());
        let count_key = DataKey::DeliveryCount(beneficiary.clone());

        let current_points: u32 = env.storage().persistent().get(&points_key).unwrap_or(0);
        let current_kg: u32 = env.storage().persistent().get(&kg_key).unwrap_or(0);
        let count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);

        let new_points = current_points.checked_add(points).ok_or(EcoError::Overflow)?;
        let new_kg = current_kg.checked_add(kilograms).ok_or(EcoError::Overflow)?;
        let next_count = count.checked_add(1).ok_or(EcoError::Overflow)?;

        env.storage().persistent().set(&points_key, &new_points);
        env.storage().persistent().set(&kg_key, &new_kg);
        env.storage().persistent().set(&count_key, &next_count);
        env.storage().persistent().set(
            &DataKey::Delivery(beneficiary.clone(), count),
            &Delivery { kilograms, points },
        );

        DeliveryRecorded {
            beneficiary,
            kilograms,
            points,
        }
        .publish(&env);

        Ok(points)
    }

   // Función para retornar los puntos acumulados por una dirección
    pub fn get_points(env: Env, beneficiary: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Points(beneficiary))
            .unwrap_or(0)
    }

   // Función para retonar la cantidad total de kilogramos reciclados por una dirección
    pub fn get_total_kg(env: Env, beneficiary: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::TotalKg(beneficiary))
            .unwrap_or(0)
    }

    // Funcion para retornar la cantidad de entregas realizadas por una dirección
    pub fn get_delivery_count(env: Env, beneficiary: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::DeliveryCount(beneficiary))
            .unwrap_or(0)
    }

    // Funcion para retornar la entrega mediante un indice de entrega basada en Cero
    pub fn get_delivery(env: Env, beneficiary: Address, index: u32) -> Option<Delivery> {
        env.storage()
            .persistent()
            .get(&DataKey::Delivery(beneficiary, index))
    }

   // Función para retornar las direcciones de un administrador
    pub fn get_admin(env: Env) -> Result<Address, EcoError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(EcoError::NotInitialized)
    }
}

#[cfg(test)]
mod test;
