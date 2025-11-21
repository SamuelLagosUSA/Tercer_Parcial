# Comparación de rendimiento entre Python y Rust

## 1. Velocidad de ejecución

| Lenguaje       | Tiempo aproximado (dataset pequeño) | Motivo                          |
|----------------|-------------------------------------|---------------------------------|
| Python (NumPy) | 0.5 ms – 2 ms                       | NumPy usa código C optimizado   |
| Rust (--release) | 0.06 ms – 0.2 ms                  | Código nativo altamente optimizado |

**Conclusión:**  
Rust es más rápido, pero NumPy reduce la diferencia. En cálculos no vectorizados o datasets grandes, Rust se vuelve mucho más veloz.

---

## 2. Uso de memoria

| Lenguaje | Memoria                                   |
|----------|-------------------------------------------|
| Python   | Alta (intérprete + objetos + NumPy)       |
| Rust     | Muy baja (binario nativo sin runtime)     |

**Conclusión:**  
Rust es mucho más eficiente en consumo de memoria.

---

## 3. Escalabilidad y paralelismo

- **Python:** depende de librerías externas para paralelismo real (el GIL limita hilos).  
- **Rust:** permite paralelismo nativo, seguro y sin overhead adicional.  

**Conclusión:**  
Rust escala mucho mejor en cargas computacionales grandes.

---

## 4. Seguridad

- **Python:** errores posibles en librerías C, no hay garantías de memoria.  
- **Rust:** el *borrow checker* elimina errores típicos y evita fallas de memoria.  

**Conclusión:**  
Rust es mucho más seguro para sistemas críticos.

---

## Resumen final

- **Python:** rápido para prototipos y cuando se usan librerías optimizadas.  
- **Rust:** más rápido, más seguro y más eficiente en producción o grandes volúmenes de datos.
