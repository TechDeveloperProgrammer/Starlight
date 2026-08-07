# Guía de Empaquetado - Starlight Core

Esta guía describe el proceso completo de empaquetado y publicación de artefactos para Starlight Core.

---

## Tabla de Contenidos

1. [Visión General](#visión-general)
2. [Paquetes Java](#paquetes-java)
3. [Paquetes Bedrock](#paquetes-bedrock)
4. [Paquetes Rust](#paquetes-rust)
5. [CI/CD Automatizado](#cicd-automatizado)
6. [Verificación de Artefactos](#verificación-de-artefactos)

---

## Visión General

Starlight Core genera tres tipos de paquetes:

| Tipo | Tecnología | Destino | Formato |
|------|-----------|---------|---------|
| Java | Gradle/Maven | GitHub Packages | `.jar`, `.pom` |
| Bedrock | CMake/C++ | GitHub Releases | `.zip`, `.mcpack` |
| Rust | Cargo | crates.io / Releases | `.crate`, `.zip` |

---

## Paquetes Java

### Configuración

Los paquetes Java se configuran en `java/build.gradle.kts`:

```kotlin
plugins {
    `java-library`
    `maven-publish`
}

group = "com.tuorganizacion.starlight"
version = project.findProperty("releaseVersion") ?: "0.0.0-dev"
```

### Artefactos Generados

1. **starlight-core-model** - Modelos de datos comunes
2. **starlight-java-adapter** - Adaptador de protocolo Java
3. **starlight-protocol-common** - Utilidades de protocolo

### Publicación en GitHub Packages

#### 1. Configurar credenciales

En tu proyecto consumidor, añade a `gradle.properties`:

```properties
gpr.user=TU_USUARIO_GITHUB
gpr.key=TU_TOKEN_CON_READ_PACKAGES
```

#### 2. Añadir repositorio

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
```

#### 3. Consumir dependencias

```kotlin
dependencies {
    implementation("com.tuorganizacion.starlight:starlight-java-adapter:0.1.0")
    implementation("com.tuorganizacion.starlight:starlight-core-model:0.1.0")
}
```

### Comandos Manuales

```bash
# Build local
cd java
./gradlew build

# Publicar localmente
./gradlew publishToMavenLocal

# Publicar en GitHub Packages (requiere GITHUB_TOKEN)
./gradlew publish
```

---

## Paquetes Bedrock

### Estructura del Paquete

El paquete Bedrock típico contiene:

```
starlight-bedrock-adapter-0.1.0-linux-x64.zip
├── bin/
│   └── starlight_bedrock_adapter
├── lib/
│   ├── libstarlight_core.so
│   └── libstarlight_bedrock_adapter.so
├── config/
│   └── starlight.toml
├── docs/
│   └── README.md
└── manifest.json
```

### Empaquetado con CPack

```bash
cd cpp
cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build
cpack -G ZIP --config build/CPackConfig.cmake
```

### Empaquetado Manual (.mcpack)

```bash
cd packaging/bedrock/mcpack
zip -r ../../starlight-bedrock-addon-0.1.0.mcpack .
sha256sum ../../starlight-bedrock-addon-0.1.0.mcpack > ../../starlight-bedrock-addon-0.1.0.mcpack.sha256
```

### Contenido de manifest.json

```json
{
  "format_version": 2,
  "header": {
    "name": "Starlight Core Bedrock Adapter",
    "description": "Capa de integración Bedrock para Starlight Core",
    "uuid": "<UUID-ÚNICO-HEADER>",
    "version": [0, 1, 0],
    "min_engine_version": [1, 20, 0]
  },
  "modules": [
    {
      "type": "data",
      "uuid": "<UUID-ÚNICO-MÓDULO>",
      "version": [0, 1, 0]
    }
  ]
}
```

> **Importante**: Los UUID deben ser únicos. Genera nuevos UUID para cada versión o variante.

---

## Paquetes Rust

### Build del Crate

```bash
cargo build --release
cargo package
```

### Publicación en crates.io (Opcional)

```bash
cargo login TU_API_TOKEN
cargo publish
```

### Artefactos para GitHub Releases

```bash
# Compilar para múltiples targets
cargo build --release --all-features

# Crear archive con FFI
mkdir starlight-core-ffi-0.1.0
cp target/release/*.so starlight-core-ffi-0.1.0/
cp target/release/*.a starlight-core-ffi-0.1.0/
tar -czvf starlight-core-ffi-0.1.0.tar.gz starlight-core-ffi-0.1.0/
sha256sum starlight-core-ffi-0.1.0.tar.gz > starlight-core-ffi-0.1.0.tar.gz.sha256
```

---

## CI/CD Automatizado

### Workflow de Release

El archivo `.github/workflows/release.yml` automatiza:

1. **Trigger**: Push de tag `v*`
2. **Java**: Build, test, publicación en GitHub Packages
3. **Rust**: Build, test, upload de artefactos
4. **C++**: Build, test, empaquetado
5. **Release**: Upload a GitHub Releases con checksums

### Ejecutar Manualmente

```bash
# Crear tag
git tag v0.1.0
git push origin v0.1.0

# GitHub Actions ejecutará automáticamente todo el pipeline
```

### Verificar Estado

Visita: https://github.com/TU_ORGANIZACION/starlight-core/actions

---

## Verificación de Artefactos

### Java

```bash
# Verificar JAR
jar tf starlight-java-adapter-0.1.0.jar

# Verificar POM
cat starlight-java-adapter-0.1.0.pom

# Verificar sources y javadoc
jar tf starlight-java-adapter-0.1.0-sources.jar
jar tf starlight-java-adapter-0.1.0-javadoc.jar
```

### Bedrock

```bash
# Verificar checksum
sha256sum -c starlight-bedrock-adapter-0.1.0-linux-x64.zip.sha256

# Listar contenido
unzip -l starlight-bedrock-adapter-0.1.0-linux-x64.zip

# Verificar manifest.json
jq . starlight-bedrock-adapter-0.1.0/manifest.json
```

### Rust

```bash
# Verificar crate
tar -tzf starlight-core-0.1.0.crate

# Verificar integridad
sha256sum -c starlight-core-0.1.0.crate.sha256
```

---

## Checklist de Verificación Pre-Release

```markdown
[ ] Todos los tests pasan (Java, Rust, C++)
[ ] Build limpio sin warnings críticos
[ ] Versiones actualizadas en todos los módulos
[ ] CHANGELOG.md actualizado
[ ] Documentación actualizada
[ ] UUIDs únicos en manifest.json
[ ] Checksums generados correctamente
[ ] Artefactos descargables
[ ] Notas de release completas
```

---

## Solución de Problemas

### Error: No se puede publicar en GitHub Packages

**Causa**: Permisos insuficientes del token.

**Solución**: Asegúrate de que `GITHUB_TOKEN` tenga permisos `packages: write`.

### Error: UUID duplicado en manifest.json

**Causa**: Reutilización de UUID entre versiones.

**Solución**: Genera nuevos UUID únicos:

```bash
# Linux
uuidgen

# Python
python -c "import uuid; print(uuid.uuid4())"
```

### Error: Checksum no coincide

**Causa**: Archivo modificado después de generar checksum.

**Solución**: Regenera checksums después del build final:

```bash
sha256sum archivo.zip > archivo.zip.sha256
```

---

## Referencias

- [GitHub Packages Documentation](https://docs.github.com/en/packages)
- [Gradle Maven Publish Plugin](https://docs.gradle.org/current/userguide/publishing_maven.html)
- [CPack Documentation](https://cmake.org/cmake/help/latest/module/CPack.html)
- [Cargo Publish](https://doc.rust-lang.org/cargo/reference/publishing.html)

