cargo build
mkdir -p ~/.nargo/backends/acvm-backend-plonky2
ln -sf $PWD/target/debug/acvm-backend-plonky2 ~/.nargo/backends/acvm-backend-plonky2/backend_binary

# no trailing new line!!1
printf acvm-backend-plonky2 > ~/.nargo/backends/.selected_backend

# use nargo backend select to revert to barrettenberg