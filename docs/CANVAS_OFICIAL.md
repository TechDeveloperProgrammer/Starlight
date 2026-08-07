# STARLIGHT CORE — CANVAS OFICIAL v2

## Nombre
**Starlight Core**

## Tipo
Bridge bidireccional Minecraft Java Edition ↔ Bedrock Edition

## Objetivo
Traducir con precisión y alto rendimiento paquetes, entidades, ítems, bloques, NBT, chunks y estados entre Java y Bedrock.

---

## Arquitectura

- **Java Adapter**: protocolo y estructuras Java Edition
- **Bedrock Adapter**: protocolo y estructuras Bedrock Edition
- **Starlight Core**: núcleo en Rust con modelo canónico y motor de traducción
- **Interop**: capa segura Java ↔ Rust ↔ C++
- **Packaging**: generación automática de artefactos Java y Bedrock

---

## Empaquetado Java

- **Build**: Gradle Kotlin DSL
- **Publicación**: GitHub Packages Maven
- **Artefactos**:
  - `starlight-core-model`
  - `starlight-java-adapter`
  - `starlight-protocol-common`
- **Coordenadas Maven**: `com.tuorganizacion.starlight:*`

---

## Empaquetado Bedrock

- **Build**: CMake/C++
- **Empaquetado**: CPack o scripts ZIP
- **Artefactos publicados**: GitHub Releases
- **Paquetes**:
  - `starlight-bedrock-adapter-<version>-linux-x64.zip`
  - `starlight-bedrock-adapter-<version>-windows-x64.zip`
  - `starlight-bedrock-addon-<version>.mcpack` (opcional)
- **Checksums**: SHA-256 para todos los artefactos

---

## Empaquetado Rust

- **Build**: Cargo
- **Publicación**: crates.io (opcional) o GitHub Releases
- **Artefactos**:
  - `starlight-core-<version>.crate`
  - `starlight-core-ffi-<version>.zip`

---

## CI/CD Automatizado

### GitHub Actions Workflows

1. **ci.yml** (Pull Requests y pushes a main/develop)
   - Lint y formato
   - Pruebas unitarias Java/Rust/C++
   - Build de todos los componentes
   - Sanitizers C++ (ASan, UBSan)
   - Clippy Rust
   - Empaquetado de prueba

2. **release.yml** (Tags v*)
   - Compilación multiplataforma
   - Publicación Java en GitHub Packages
   - Generación de paquetes Bedrock
   - Upload a GitHub Releases
   - Generación de checksums SHA-256
   - Notas de release automáticas

---

## Calidad

- ✅ Pruebas unitarias obligatorias
- ✅ Pruebas de integración obligatorias
- ✅ Pruebas de protocolo
- ✅ Fuzzing continuo
- ✅ Sanitizers en C++
- ✅ Clippy y tests en Rust
- ✅ Benchmarks de rendimiento
- ✅ Checksums SHA-256 para paquetes

---

## Filosofía

1. **Precisión primero** - La traducción debe ser fiel al comportamiento original
2. **Estabilidad siempre** - Cero crashes en producción
3. **Rendimiento medible** - Benchmarks continuos y optimización basada en datos
4. **Ninguna traducción crítica sin pruebas** - Validación obligatoria
5. **Ningún release sin paquetes verificables** - Artefactos comprobables

---

## Meta de Calidad

**Cero defectos críticos conocidos** mediante:
- Arquitectura canónica robusta
- Validación continua automatizada
- Releases auditables con checksums
- Monitoreo y feedback constante

---

## Flujo de Traducción

```
Java → Modelo Canónico → Bedrock
Bedrock → Modelo Canónico → Java
```

El modelo canónico actúa como intermediario neutral, permitiendo:
- Traducciones consistentes
- Validación centralizada
- Menor complejidad (N+M vs N*M)

---

## Proceso de Release

1. Crear tag semántico: `git tag v0.1.0`
2. Push del tag: `git push origin v0.1.0`
3. GitHub Actions dispara automáticamente:
   - Build y test de todos los componentes
   - Publicación Java en GitHub Packages
   - Empaquetado Bedrock
   - Upload a GitHub Releases con checksums
4. Verificar artefactos publicados
5. Actualizar documentación si es necesario

---

## Versionado Semántico

```
MAJOR.MINOR.PATCH
```

- **MAJOR**: Cambios incompatibles en el protocolo o API
- **MINOR**: Nuevas características compatibles
- **PATCH**: Correcciones de bugs compatibles

Ejemplos:
- `v0.1.0` - Beta inicial
- `v0.2.0` - Mejoras y nuevas traducciones
- `v1.0.0` - Release estable

---

## Roles del Equipo

1. **Java Protocol Engineer** - Adaptador Java Edition y publicación Maven
2. **C++ Bedrock Engineer** - Adaptador Bedrock y empaquetado
3. **Rust Core Engineer** - Núcleo de traducción y FFI
4. **Interop Engineer** - Integración Java ↔ Rust ↔ C++
5. **Protocol Translation Specialist** - Mapeo semántico Java ↔ Bedrock
6. **Performance Engineer** - Optimización y benchmarks
7. **QA & Reliability Engineer** - Testing, fuzzing y validación
8. **Release & Packaging Engineer** - CI/CD y automatización

---

## Definición de "Hecho" para Paquetes

Un paquete solo está terminado si:

1. ✅ Compila sin errores
2. ✅ Pasa pruebas unitarias
3. ✅ Pasa pruebas de integración
4. ✅ Pasa pruebas de protocolo
5. ✅ No introduce regresiones
6. ✅ Tiene versión correcta
7. ✅ Se genera en CI
8. ✅ Se publica en el registry correcto
9. ✅ Tiene checksum
10. ✅ Es descargable/consumible
11. ✅ La documentación indica cómo usarlo

---

## Checklist de Release

```markdown
[ ] Tag creado con versión correcta
[ ] CI en verde
[ ] Java publicado en GitHub Packages
[ ] Bedrock empaquetado en GitHub Releases
[ ] Rust compilado y probado
[ ] C++ probado y empaquetado
[ ] Checksums generados
[ ] Notas de release publicadas
[ ] Artefactos descargables
[ ] Documentación actualizada
[ ] Paquete Java consumible desde proyecto de prueba
[ ] Paquete Bedrock verificable
```

---

## Enlaces Útiles

- [Arquitectura](docs/architecture.md)
- [Guía de Empaquetado](docs/packaging.md)
- [GitHub Packages Setup](docs/github-packages.md)
- [Mapeo Java ↔ Bedrock](docs/java_bedrock_mapping.md)
- [Proceso de Release](docs/release_process.md)

---

**Starlight Core** - Conectando mundos, un paquete a la vez. ✨
