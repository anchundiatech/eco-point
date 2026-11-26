# EcoPuntos CR

## Problema

En algunas comunidades de Costa Rica, las asociaciones y pequeños centros de acopio organizan jornadas de reciclaje. Llevar el control de las entregas y de los reconocimientos de cada participante puede hacerse con registros manuales, lo que dificulta revisar el historial y comprobar los puntos acumulados.

## Solución

EcoPuntos CR es un contrato inteligente de Stellar Soroban que registra las entregas de material reciclable por participante y calcula puntos automáticamente.

**Regla:** cada kilogramo registrado equivale a 10 puntos.

El contrato no recibe ni transfiere dinero. Los puntos son únicamente un registro contable dentro del contrato; no son un token ni tienen valor monetario.

## Cómo funciona

1. Una persona administradora inicializa el contrato con su dirección Stellar.
2. Cuando alguien entrega material reciclable, la persona administradora registra la dirección del participante y los kilogramos.
3. El contrato valida que esté inicializado, que la llamada esté autorizada y que los kilogramos sean mayores que cero.
4. El contrato calcula los puntos, actualiza los acumulados y guarda el detalle de la entrega.
5. Cualquier cuenta puede consultar puntos, kilogramos, cantidad de entregas y cada entrega individual.

## Por qué elegimos este problema

Elegimos una actividad comunitaria cotidiana y fácil de entender. El reciclaje permite mostrar un uso práctico de Soroban sin construir una aplicación financiera compleja. Un registro verificable puede ayudar a una asociación a llevar cuentas consistentes y dar transparencia al reconocimiento de los participantes.

El contrato no verifica físicamente el peso ni la composición del material. La persona administradora sigue siendo responsable de comprobar la entrega antes de registrarla.

## Funciones del contrato

| Función | Descripción |
|---|---|
| `initialize(admin)` | Configura la cuenta administradora una sola vez. |
| `record_delivery(beneficiary, kilograms)` | Registra una entrega y devuelve los puntos asignados. |
| `get_points(beneficiary)` | Consulta los puntos acumulados. |
| `get_total_kg(beneficiary)` | Consulta los kilogramos acumulados. |
| `get_delivery_count(beneficiary)` | Consulta cuántas entregas tiene registradas. |
| `get_delivery(beneficiary, index)` | Consulta una entrega; el índice comienza en cero. |
| `get_admin()` | Consulta la cuenta administradora. |

## Estructura

```text
eco-puntos-cr/
├── Cargo.toml
└── contracts/
    └── eco-puntos/
        ├── Cargo.toml
        └── src/
            ├── lib.rs
            └── test.rs
```

## Requisitos

- Rust y Cargo.
- Stellar CLI.
- Target de compilación `wasm32v1-none`.

Instala el target si todavía no está disponible:

```bash
rustup target add wasm32v1-none
```

## Cómo probar

Desde la raíz del repositorio:

```bash
cargo test --workspace
```

Las pruebas cubren:

1. Inicialización y almacenamiento de la cuenta administradora.
2. Registro de una entrega de 5 kg y asignación de 50 puntos.
3. Acumulación de dos entregas.
4. Rechazo de una entrega de cero kilogramos.
5. Rechazo de una segunda inicialización.
6. Rechazo de entregas antes de inicializar el contrato.

## Cómo compilar para Stellar

Desde la raíz del repositorio:

```bash
stellar contract build
```

El archivo WebAssembly se genera en:

```text
target/wasm32v1-none/release/eco_puntos.wasm
```

## Alcance y limitaciones

- Es una práctica académica individual.
- No incluye una interfaz web.
- No crea ni transfiere tokens.
- No verifica básculas ni entregas físicas.
- La cuenta administradora es responsable de registrar datos correctos.
- Los datos se consultan desde el contrato; no se almacenan nombres, teléfonos ni otros datos personales.

## Red

Las pruebas se ejecutan localmente con el entorno de pruebas de Soroban. La compilación genera el WASM para desplegarlo posteriormente en Stellar Testnet. Este repositorio no incluye una dirección de contrato desplegado.
