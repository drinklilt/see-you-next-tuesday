# See You Next Tuesday
See You Next Tuesday is a web service that returns the time until the next tuesday

## Endpoints
 - GET /api/ns - Returns the time until the next tuesday in nanoseconds  

   **Request**  
   `curl -s -X GET "${endpoint}/api/ns"`  

   **Response**  
   `128512154427300`

 - GET /api/ms - Returns the time until the next tuesday in milliseconds  
  
   **Request**  
   `curl -s -X GET "${endpoint}/api/ms"`  

   **Response**  
   `128425049`

 - GET /api/s - Returns the time until the next tuesday in seconds  
 
   **Request**  
   `curl -s -X GET "${endpoint}/api/s"`  

   **Response**  
   `128435`