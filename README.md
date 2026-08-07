# STARLIGHT CORE

## Bridge bidireccional Java Edition ↔ Bedrock Edition
### Preciso, optimizado, estable y empaquetado automáticamente con GitHub Actions / Packages

[![CI](https://github.com/TU_ORGANIZACION/starlight-core/actions/workflows/ci.yml/badge.svg)](https://github.com/TU_ORGANIZACION/starlight-core/actions/workflows/ci.yml)
[![Release](https://github.com/TU_ORGANIZACION/starlight-core/actions/workflows/release.yml/badge.svg)](https://github.com/TU_ORGANIZACION/starlight-core/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## 🌟 Visión General

**Starlight Core** es un bridge bidireccional profesional entre **Minecraft Java Edition** y **Minecraft Bedrock Edition**, diseñado para:

- ✅ Traducir protocolos con alta fidelidad
- ✅ Mantener rendimiento optimizado y bajo overhead
- ✅ Garantizar estabilidad de producción
- ✅ Generar paquetes automáticos para Java y Bedrock
- ✅ Publicar artefactos versionados en GitHub Packages y Releases

---

## 🏗️ Arquitectura

```
┌──────────────────────┐         ┌──────────────────────┐
│   Java Edition       │         │   Bedrock Edition    │
│   Adapter (Java)     │         │   Adapter (C++)      │
└──────────┬───────────┘         └───────────┬──────────┘
           │                                 │
           ▼                                 ▼
┌──────────────────────────────────────────────────────────┐
│                    STARLIGHT CORE                        │
│                      (Rust Core)                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │   Protocol   │  │  Translation │  │   Canonical  │   │
│  │   Decoder    │  │    Engine    │  │  World State │   │
│  └──────────────┘  └──────────────┘  └──────────────┘   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │  Packet      │  │ Entity/Item  │  │    NBT       │   │
│  │   Router     │  │   Mapper     │  │   Mapper     │   │
│  └──────────────┘  └──────────────┘  └──────────────┘   │
└──────────────────────────────────────────────────────────┘
```

---

## 📦 Paquetes Disponibles

### Java (GitHub Packages Maven)

```kotlin
repositories {
    maven {
        url = uri("https://maven.pkg.github.com/TU_ORGANIZACION/starlight-core")
        credentials {
            username = providers.gradleProperty("gpr.user").get()
            password = providers.gradleProperty("gpr.key").get()
        }
    }
}

dependencies {
    implementation("com.tuorganizacion.starlight:starlight-java-adapter:0.1.0")
    implementation("com.tuorganizacion.starlight:starlight-core-model:0.1.0")
    implementation("com.tuorganizacion.starlight:starlight-protocol-common:0.1.0")
}
```

### Bedrock (GitHub Releases)

- `starlight-bedrock-adapter-<version>-linux-x64.zip`
- `starlight-bedrock-adapter-<version>-windows-x64.zip`
- `starlight-bedrock-addon-<version>.mcpack` (opcional)

Todos los artefactos incluyen checksums SHA-256 para verificación.

---

## 🚀 Inicio Rápido

### Requisitos Previos

- **Java**: JDK 21+
- **Rust**: Rust stable (rustup recomendado)
- **C++**: CMake 3.25+, Ninja, g++ con soporte C++20
- **Gradle**: 8.x (wrapper incluido)

### Build del Proyecto

```bash
# Java
cd java
./gradlew build

# Rust
cargo build --release

# C++
cmake -S cpp -B build -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build
```

### Ejecutar Pruebas

```bash
# Java
./gradlew test

# Rust
cargo test --all-features

# C++
ctest --test-dir build --output-on-failure
```

---

## 🎯 Principios de Desarrollo

1. **Precisión ante todo** - La traducción debe ser fiel al comportamiento original
2. **Estabilidad obligatoria** - Cero crashes en producción
3. **Rendimiento medible** - Benchmarks continuos y optimización basada en datos
4. **Memoria segura** - Sin fugas, sin comportamiento indefinido
5. **Errores controlados** - Nunca fallos silenciosos
6. **Pruebas obligatorias** - Ninguna traducción crítica sin tests
7. **Paquetes verificables** - Cada release genera artefactos comprobables
8. **Compatibilidad versionada** - Semantic Versioning estricto
9. **Automatización total** - CI/CD para todo el flujo
10. **Cero defectos críticos** - Objetivo mediante arquitectura y validación continua

---

## 📁 Estructura del Repositorio

```
starlight-core/
├── .github/workflows/       # CI/CD pipelines
├── crates/                  # Componentes Rust
│   ├── starlight-core/
│   ├── starlight-ffi/
│   └── ...
├── java/                    # Componentes Java
│   ├── modules/
│   │   ├── starlight-core-model/
│   │   ├── starlight-java-adapter/
│   │   └── starlight-protocol-common/
│   └── build.gradle.kts
├── cpp/                     # Componentes C++
│   ├── src/
│   ├── include/
│   └── CMakeLists.txt
├── packaging/               # Configuración de empaquetado
│   ├── bedrock/
│   ├── java/
│   └── rust/
├── tests/                   # Suite de pruebas
│   ├── protocol/
│   ├── integration/
│   ├── e2e/
│   └── fuzz/
└── docs/                    # Documentación
```

---

## 🔧 CI/CD Automatizado

### Pipeline de CI (Pull Requests)

- ✅ Lint y formato de código
- ✅ Pruebas unitarias
- ✅ Build de todos los componentes
- ✅ Sanitizers (ASan, UBSan) para C++
- ✅ Clippy para Rust
- ✅ Empaquetado de prueba

### Pipeline de Release (Tags v*)

- 📦 Compilación multiplataforma
- 📦 Publicación Java en GitHub Packages
- 📦 Generación de paquetes Bedrock
- 📦 Upload a GitHub Releases
- 📦 Generación de checksums SHA-256
- 📦 Notas de release automáticas

---

## 📊 Matriz de Compatibilidad

| Versión | Java Edition | Bedrock Edition | Estado |
|---------|--------------|-----------------|--------|
| 0.1.0   | 1.20.x       | 1.20.x          | 🟡 Beta |
| 0.2.0   | 1.20.x       | 1.20.x          | 🟡 Beta |
| 1.0.0   | 1.21.x       | 1.21.x          | 🔵 Planned |

---

## 🧪 Calidad y Testing

- **Unit Tests**: Pruebas por componente
- **Integration Tests**: Validación de interoperabilidad
- **E2E Tests**: Pruebas de extremo a extremo
- **Fuzzing**: Detección de edge cases
- **Sanitizers**: ASan, UBSan, MSan para C++
- **Benchmarks**: Medición continua de rendimiento

---

## 📝 Roles del Equipo

1. **Java Protocol Engineer** - Adaptador Java Edition
2. **C++ Bedrock Engineer** - Adaptador Bedrock Edition
3. **Rust Core Engineer** - Núcleo de traducción
4. **Interop Engineer** - FFI Java ↔ Rust ↔ C++
5. **Protocol Translation Specialist** - Mapeo semántico
6. **Performance Engineer** - Optimización y benchmarks
7. **QA & Reliability Engineer** - Testing y fuzzing
8. **Release & Packaging Engineer** - CI/CD y empaquetado

---

## 📄 Licencia

MIT License - ver [LICENSE](LICENSE) para detalles.

---

## 🔗 Enlaces Útiles

- [Documentación de Arquitectura](docs/architecture.md)
- [Guía de Empaquetado](docs/packaging.md)
- [GitHub Packages Setup](docs/github-packages.md)
- [Mapeo Java ↔ Bedrock](docs/java_bedrock_mapping.md)
- [Proceso de Release](docs/release_process.md)

---

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor:

1. Abre un issue discutiendo el cambio
2. Crea una rama feature
3. Asegura que todas las pruebas pasan
4. Envía un pull request

---

**Starlight Core** - Conectando mundos, un paquete a la vez. ✨
