# Proxy Inverso en Rust

Este proyecto es una implementación de un proxy inverso en Rust. Un proxy inverso es un servidor que se sitúa entre los clientes y uno o más servidores backend, redirigiendo las solicitudes de los clientes a los servidores adecuados y devolviendo las respuestas al cliente.

## Características

- **Balanceo de carga**: Distribuye las solicitudes entre múltiples servidores backend.
- **Configuración flexible**: Permite la configuración de múltiples servidores backend a través de un archivo de configuración.
- **Alto rendimiento**: Aprovecha la eficiencia y seguridad de Rust para manejar un alto volumen de solicitudes.
- **Fácil de usar**: Fácil de configurar y ejecutar.

## Requisitos

- Rust 1.56 o superior (para soporte de edición 2021)
- Cargo (el gestor de paquetes de Rust)

## Instalación

1. Clona el repositorio:

   ```bash
   git clone https://github.com/tuusuario/proxy-inverso-rust.git
   cd proxy-inverso-rust
   
2. Compila el proyecto:
 ```bash
    cargo build --release
```
# Configuración
El proxy inverso se configura mediante un archivo config.yml. Aquí tienes un ejemplo de configuración:
``` yml
microservices:
  - path_prefix: "/users"
    instances:
      - "http://localhost:3200"
      - "http://localhost:3201"
  - path_prefix: "/orders"
    instances:
        - "http://localhost:3500"
        - "http://localhost:3501"
rate_limit: 100
```

# Contribución
¡Las contribuciones son bienvenidas! Si deseas contribuir al proyecto, por favor sigue estos pasos:

Haz un fork del repositorio.

Crea una rama para tu feature (git checkout -b feature/nueva-funcionalidad).

Haz commit de tus cambios (git commit -am 'Añade nueva funcionalidad').

Haz push a la rama (git push origin feature/nueva-funcionalidad).

Abre un Pull Request.