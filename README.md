## Configuración y ejecución del proyecto IOTA

Este proyecto utiliza los programas "Author" y "Subscriptor" para establecer una conexión a través del Tangle de IOTA. Para ejecutar el proyecto, es necesario tener ambos programas en funcionamiento simultáneamente. A continuación, se detallan los pasos necesarios:

1. **Configuración inicial**

   Clona el repositorio del proyecto en tu máquina local:
   
   
2. **Ejecución del programa Author**

Accede al directorio del programa "Author": cd proyecto-iota/author

Ejecuta el siguiente comando para iniciar el programa: cargo run

El programa "Author" generará un enlace de anuncio (announcement link) como resultado de su ejecución. Ten en cuenta este enlace, ya que será necesario para la configuración del programa "Subscriptor".

3. **Ejecución del programa Subscriptor**

Accede al directorio del programa "Subscriptor": cd ../subscriptor

Ejecuta el siguiente comando para iniciar el programa: cargo run

Durante la ejecución del programa "Subscriptor", se te proporcionará un enlace de suscripción (subscription link). Debes copiar este enlace, ya que lo necesitarás en el siguiente paso.

4. **Configuración de la conexión entre Author y Subscriptor**

Vuelve al directorio del programa "Author":


Abre el archivo de configuración `archivo.txt` y pega el enlace de suscripción (subscription link) que copiaste en el paso anterior.

Guarda los cambios en el archivo de configuración.

5. **Finalización de la conexión y transferencia de datos**

Una vez hayas pegado el enlace de suscripción en el archivo de configuración del programa "Author", la conexión entre "Author" y "Subscriptor" estará configurada.

Los datos que se envíen a través del Tangle se almacenarán en el archivo `archivo.txt` dentro del directorio `author`.

¡Listo! Ahora tienes la conexión establecida entre el programa "Author" y "Subscriptor" a través del Tangle de IOTA. Los datos transmitidos estarán disponibles en el archivo `archivo.txt` dentro del directorio `author` del proyecto.

Recuerda que estos son solo los pasos generales para configurar y ejecutar el proyecto. Asegúrate de tener todas las dependencias necesarias instaladas y cualquier otra configuración específica del proyecto antes de ejecutarlo.

Espero que esta información te sea útil. Si tienes alguna otra pregunta, no dudes en preguntar.
