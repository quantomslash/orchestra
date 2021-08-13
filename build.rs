fn build_exchange_grpc() {
    tonic_build::configure()
        .out_dir("src/rpc/exchange")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["proto/exchange/matchengine.proto"],
            &["proto/exchange", "proto/third_party/googleapis"],
        )
        .unwrap();
}

fn main() {
    build_exchange_grpc();
}
