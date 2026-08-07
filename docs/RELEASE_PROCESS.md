# Proceso de Release - Starlight Core

Este documento describe el proceso completo para crear y publicar una nueva versión de Starlight Core.

---

## Tabla de Contenidos

1. [Preparación](#preparación)
2. [Creación del Release](#creación-del-release)
3. [Verificación](#verificación)
4. [Post-Release](#post-release)
5. [Rollback](#rollback)

---

## Preparación

### 1. Verificar Estado del Proyecto

```bash
# Asegurar que estás en la rama correcta
git checkout main
git pull origin main

# Verificar que CI está en verde
# Visita: https://github.com/TU_ORGANIZACION/starlight-core/actions

# Ejecutar tests localmente
cd java && ./gradlew test
cd ../cpp && cmake --build build --target test
cargo test --all-features
```

### 2. Actualizar Versiones

Actualiza las versiones en todos los módulos:

**Java** (`java/gradle.properties`):
```properties
version=0.2.0
```

**C++** (`cpp/CMakeLists.txt`):
```cmake
project(StarlightBedrockAdapter VERSION 0.2.0 LANGUAGES CXX)
```

**Rust** (`crates/starlight-core/Cargo.toml`):
```toml
[package]
name = "starlight-core"
version = "0.2.0"
```

### 3. Actualizar CHANGELOG.md

```markdown
## [0.2.0] - YYYY-MM-DD

### Added
- Nueva característica X
- Soporte para protocolo Y

### Changed
- Mejora en rendimiento de traducción
- Actualización de dependencias

### Fixed
- Bug en traducción de entidades
- Problema de memoria en C++

### Breaking Changes
- Cambio incompatible Z (si aplica)
```

### 4. Checklist Pre-Release

```markdown
[ ] Todos los tests pasan
[ ] No hay warnings críticos en el build
[ ] CHANGELOG.md actualizado
[ ] Versiones actualizadas en todos los módulos
[ ] Documentación actualizada
[ ] UUIDs únicos en manifest.json (Bedrock)
[ ] Revisión de código completada
```

---

## Creación del Release

### Método Automático (Recomendado)

#### 1. Crear Tag Semántico

```bash
# Crear tag con mensaje descriptivo
git tag -a v0.2.0 -m "Release v0.2.0 - Descripción breve"

# Push del tag
git push origin v0.2.0
```

#### 2. GitHub Actions Ejecuta Automáticamente

El workflow `release.yml` se dispara automáticamente:

1. ✅ Build de Java, Rust y C++
2. ✅ Tests en todos los componentes
3. ✅ Publicación Java en GitHub Packages
4. ✅ Empaquetado Bedrock
5. ✅ Upload a GitHub Releases
6. ✅ Generación de checksums

#### 3. Monitorear Progreso

Visita: https://github.com/TU_ORGANIZACION/starlight-core/actions/workflows/release.yml

---

### Método Manual (Alternativo)

Si necesitas más control:

#### 1. Publicar Java Manualmente

```bash
cd java
export GITHUB_ACTOR=tu_usuario
export GITHUB_TOKEN=ghp_tu_token_con_permisos
./gradlew publish --no-daemon -PreleaseVersion=0.2.0
```

#### 2. Build y Paquete Bedrock

```bash
cd cpp
cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release -DSTARLIGHT_VERSION=0.2.0
cmake --build build
ctest --test-dir build
cpack -G ZIP --config build/CPackConfig.cmake

# Generar checksum
sha256sum starlight-bedrock-adapter-0.2.0-linux-x64.zip > starlight-bedrock-adapter-0.2.0-linux-x64.zip.sha256
```

#### 3. Crear Release en GitHub

```bash
# Usando gh CLI
gh release create v0.2.0 \
  --title "Starlight Core v0.2.0" \
  --notes-file CHANGELOG.md \
  starlight-bedrock-adapter-0.2.0-*.zip \
  starlight-bedrock-adapter-0.2.0-*.zip.sha256 \
  starlight-bedrock-addon-0.2.0.mcpack \
  starlight-bedrock-addon-0.2.0.mcpack.sha256
```

---

## Verificación

### 1. Verificar GitHub Packages

Visita: https://github.com/TU_ORGANIZACION/starlight-core/packages

Deberías ver:
- `starlight-core-model`
- `starlight-java-adapter`
- `starlight-protocol-common`

### 2. Consumir Paquete Java (Prueba)

Crea un proyecto de prueba:

```kotlin
// build.gradle.kts
repositories {
    maven {
        url = uri("https://maven.pkg.github.com/TU_ORGANIZACION/starlight-core")
        credentials {
            username = "tu_usuario"
            password = "tu_token"
        }
    }
}

dependencies {
    implementation("com.tuorganizacion.starlight:starlight-java-adapter:0.2.0")
}
```

```bash
./gradlew build
```

### 3. Verificar GitHub Releases

Visita: https://github.com/TU_ORGANIZACION/starlight-core/releases

Deberías ver:
- Release `v0.2.0` creado
- Artefactos Bedrock disponibles
- Checksums publicados

### 4. Verificar Checksums

```bash
# Descargar artefacto y checksum
wget https://github.com/TU_ORGANIZACION/starlight-core/releases/download/v0.2.0/starlight-bedrock-adapter-0.2.0-linux-x64.zip
wget https://github.com/TU_ORGANIZACION/starlight-core/releases/download/v0.2.0/starlight-bedrock-adapter-0.2.0-linux-x64.zip.sha256

# Verificar
sha256sum -c starlight-bedrock-adapter-0.2.0-linux-x64.zip.sha256
```

### 5. Checklist Post-Release

```markdown
[ ] Java publicado en GitHub Packages
[ ] Paquetes Java consumibles
[ ] Bedrock empaquetado en GitHub Releases
[ ] Checksums verificados
[ ] Notas de release publicadas
[ ] Tags creados correctamente
[ ] CI/CD en verde
```

---

## Post-Release

### 1. Anunciar Release

- Actualizar README.md si es necesario
- Publicar en canales de comunicación
- Notificar a usuarios clave

### 2. Actualizar Versión de Desarrollo

Incrementa la versión a la siguiente versión de desarrollo:

**Java**: `0.3.0-SNAPSHOT`
**C++**: `0.3.0`
**Rust**: `0.3.0-alpha`

### 3. Monitorear Issues

Vigila posibles problemas reportados:
- https://github.com/TU_ORGANIZACION/starlight-core/issues

---

## Rollback

Si necesitas revertir un release problemático:

### 1. Eliminar Tag

```bash
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0
```

### 2. Eliminar Release en GitHub

```bash
gh release delete v0.2.0 --cleanup-tag
```

### 3. Eliminar Paquetes Java (GitHub Packages UI)

Visita: https://github.com/TU_ORGANIZACION/starlight-core/packages

- Selecciona el paquete
- Click en "Delete package"

### 4. Corregir Problemas

- Crear rama de hotfix
- Resolver issues
- Crear nuevo release `v0.2.1`

---

## Versionado Semántico

Starlight Core sigue Semantic Versioning 2.0.0:

```
MAJOR.MINOR.PATCH
```

### Cuándo incrementar cada número:

- **MAJOR**: Cambios incompatibles hacia atrás
  - Cambios breaking en API
  - Cambios incompatibles en protocolo
  
- **MINOR**: Nuevas características compatibles
  - Nuevos features
  - Funcionalidad adicional
  - Deprecaciones (aún compatibles)
  
- **PATCH**: Correcciones de bugs compatibles
  - Bug fixes
  - Mejoras de rendimiento
  - Parches de seguridad

### Ejemplos:

- `0.1.0` → Beta inicial
- `0.1.1` → Bug fix en beta
- `0.2.0` → Nueva característica
- `1.0.0` → Release estable
- `1.0.1` → Patch de estabilidad
- `2.0.0` → Breaking changes

---

## Referencias

- [Semantic Versioning 2.0.0](https://semver.org/)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)
- [GitHub Packages](https://docs.github.com/en/packages)
- [Git Tags](https://git-scm.com/book/en/v2/Git-Basics-Tagging)

