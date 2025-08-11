use std::collections::{HashMap, HashSet};

struct MobileAppMonitor {
    app_name: String,
    metrics: HashMap<String, f64>,
    anomaly_detection_threshold: f64,
    ai_model: ArtificialIntelligenceModel,
}

struct ArtificialIntelligenceModel {
    trained_data: Vec<f64>,
}

impl MobileAppMonitor {
    fn new(app_name: String) -> Self {
        MobileAppMonitor {
            app_name,
            metrics: HashMap::new(),
            anomaly_detection_threshold: 3.0,
            ai_model: ArtificialIntelligenceModel {
                trained_data: Vec::new(),
            },
        }
    }

    fn collect_metric(&mut self, metric_name: &str, value: f64) {
        self.metrics.insert(metric_name.to_string(), value);
    }

    fn train_ai_model(&mut self) {
        let mut trained_data: Vec<f64> = self.metrics.values().cloned().collect();
        trained_data.sort_unstable();
        self.ai_model.trained_data = trained_data;
    }

    fn detect_anomaly(&self, new_metric: &str, new_value: f64) -> bool {
        let mut new_data: Vec<f64> = self.ai_model.trained_data.clone();
        new_data.push(new_value);
        new_data.sort_unstable();
        let q1 = new_data[new_data.len() / 4];
        let q3 = new_data[3 * new_data.len() / 4];
        let iqr = q3 - q1;
        let lower_bound = q1 - (1.5 * iqr);
        let upper_bound = q3 + (1.5 * iqr);
        new_value < lower_bound || new_value > upper_bound
    }
}

fn main() {
    let mut monitor = MobileAppMonitor::new("MyAwesomeApp".to_string());
    monitor.collect_metric("cpu_usage", 20.0);
    monitor.collect_metric("memory_usage", 30.0);
    monitor.collect_metric("response_time", 50.0);
    monitor.train_ai_model();

    let new_metric = "response_time";
    let new_value = 100.0;
    let is_anomaly = monitor.detect_anomaly(new_metric, new_value);
    println!("Is anomaly: {}", is_anomaly);
}