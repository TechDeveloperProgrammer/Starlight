plugins {
    `java-library`
    `maven-publish`
}

group = "com.tuorganizacion.starlight"
version = project.findProperty("releaseVersion") ?: "0.0.0-dev"

repositories {
    mavenCentral()
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(21))
    }
    withSourcesJar()
    withJavadocJar()
}

publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            from(components["java"])
            pom {
                name.set("Starlight Core Java")
                description.set("Adaptador y modelos Java para Starlight Core")
                url.set("https://github.com/TU_ORGANIZACION/starlight-core")
                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }
                developers {
                    developer {
                        id.set("starlight-team")
                        name.set("Starlight Core Team")
                    }
                }
                scm {
                    connection.set("scm:git:https://github.com/TU_ORGANIZACION/starlight-core.git")
                    developerConnection.set("scm:git:ssh://github.com/TU_ORGANIZACION/starlight-core.git")
                    url.set("https://github.com/TU_ORGANIZACION/starlight-core")
                }
            }
        }
    }
    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/TU_ORGANIZACION/starlight-core")
            credentials {
                username = System.getenv("GITHUB_ACTOR") ?: ""
                password = System.getenv("GITHUB_TOKEN") ?: ""
            }
        }
    }
}
