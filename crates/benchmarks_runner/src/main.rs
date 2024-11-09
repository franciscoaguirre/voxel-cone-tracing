use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Benchmark {
    pub config: String,
    pub scene: String,
    pub preset: String,
    pub seconds_for_fps: u32,
    pub number_of_builds: u32,
}

/// Loads the benchmarks from `benchmarks.ron`
fn load_benchmarks() -> Vec<Benchmark> {
    let input_path = "benchmarks.ron";
    let file = File::open(&input_path).expect("Missing benchmarks file!");
    let benchmarks: Vec<Benchmark> =
        ron::de::from_reader(file).expect("Benchmarks file malformed!");
    benchmarks
}

fn main() {
    let benchmarks = load_benchmarks();

    // Proceso de correr benchmarks:
    // 0) Loopear por cada una de las benchmarks definidas.
    // 1) Crear un archivo específico para esta benchmark para guardar resultados.
    // 2) Cargar config, scene y preset de acá en lugar de las flags.
    // 3) Construir el octree e inyectar luz `number_of_builds` veces, promediar y guardar.
    // 4) Seguir la ejecución del programa con la última build del octree.
    // 5) Sacar una foto de la escena y guardar.
    // 6) Contar FPS por `seconds_for_fps` segundos, promediar y guardar.

    for benchmark in benchmarks.iter() {
        let name = format!(
            "benchmarks/{}_{}_{}",
            &benchmark.config, &benchmark.scene, &benchmark.preset
        );
        println!("Running benchmark {}", name);

        let _ = std::fs::create_dir(&name); // We don't handle error because it errors if folder already exists

        // First take a screenshot
        println!("Taking screenshot");
        let _ = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("--")
            .arg("--config")
            .arg(&benchmark.config)
            .arg("--scene")
            .arg(&benchmark.scene)
            .arg("--preset")
            .arg(&benchmark.preset)
            .arg("--screenshot")
            .output()
            .expect("Failed to execute process!");

        // Second get the average FPS
        println!("Doing the FPS for {} seconds", benchmark.seconds_for_fps);
        let _ = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("--")
            .arg("--config")
            .arg(&benchmark.config)
            .arg("--scene")
            .arg(&benchmark.scene)
            .arg("--preset")
            .arg(&benchmark.preset)
            .arg("--seconds-for-fps")
            .arg(&benchmark.seconds_for_fps.to_string())
            .output()
            .expect("Failed to execute process!");

        // Third get the average octree build time
        println!("Starting to record the octree build time");
        for index in 0..benchmark.number_of_builds {
            println!("Execution number {}", index);
            let _ = Command::new("cargo")
                .arg("run")
                .arg("--release")
                .arg("--")
                .arg("--config")
                .arg(&benchmark.config)
                .arg("--scene")
                .arg(&benchmark.scene)
                .arg("--preset")
                .arg(&benchmark.preset)
                .arg("--record-octree-build-time")
                .output()
                .expect("Failed to execute process!");
        }

        // Now that all the octree build times have been written, we can average them
        println!("Averaging all octree build times");
        let octree_build_times_file_name = format!("{name}/octree_build_time.txt");
        let octree_build_times_file = File::open(&octree_build_times_file_name)
            .expect("Couldn't open octree build times file");
        let octree_build_times_sum = BufReader::new(octree_build_times_file)
            .lines()
            .map(|line| line.unwrap())
            .map(|line| {
                let build_time: f64 = line.parse().unwrap();
                println!("Build time: {build_time}");
                build_time
            })
            .sum::<f64>();
        let octree_build_time_average = octree_build_times_sum / benchmark.number_of_builds as f64;
        println!("Average: {octree_build_time_average}");
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&octree_build_times_file_name)
            .expect("Couldn't open octree build times file")
            .write_all(octree_build_time_average.to_string().as_bytes())
            .expect("Couldn't write to file");
    }
}
