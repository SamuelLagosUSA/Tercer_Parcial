use std::time::Instant;

fn main() {
    // Datos de ejemplo
    let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    // Parámetros
    let mut w = 0.0;
    let mut b = 0.0;

    // Hiperparámetros
    let learning_rate = 0.01;
    let epochs = 1000;
    let m = x.len() as f64;

    let start = Instant::now();

    for epoch in 0..epochs {
        // Predicciones
        let y_pred: Vec<f64> = x.iter().map(|xi| w * xi + b).collect();

        // Error
        let error: Vec<f64> = y_pred.iter().zip(y.iter()).map(|(p, t)| p - t).collect();

        // Gradientes
        let dw = (2.0 / m) * error.iter().zip(x.iter()).map(|(e, xi)| e * xi).sum::<f64>();
        let db = (2.0 / m) * error.iter().sum::<f64>();

        // Actualización
        w -= learning_rate * dw;
        b -= learning_rate * db;

        if (epoch + 1) % 200 == 0 {
            let mse = error.iter().map(|e| e * e).sum::<f64>() / m;
            println!("Epoch {}, MSE: {:.4}, w: {:.4}, b: {:.4}", epoch + 1, mse, w, b);
        }
    }

    let duration = start.elapsed();
    println!("\nModelo entrenado:");
    println!("w ≈ {:.4}, b ≈ {:.4}", w, b);
    println!("Tiempo total: {:?}", duration);

    // Probar el modelo
    let x_new = 7.0;
    let y_pred_new = w * x_new + b;
    println!("Para x = {}, y_pred ≈ {:.4}", x_new, y_pred_new);
}
