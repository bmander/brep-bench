# Every kernel's toolchain in one image, so that the whole suite can be built and run on one
# machine: Python 3.12 with Open CASCADE (OCP) and SMLib (usd-brep), stable Rust, and .NET 10.
# The repository is not copied in: docker/bench.sh mounts it, so the runners build into it and
# the results land in it (see docs/running.md).
FROM python:3.12-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential clang cmake curl ca-certificates git pkg-config \
        libgl1 libglib2.0-0 libx11-6 libxext6 libxrender1 libfontconfig1 libfreetype6 libicu72 \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    DOTNET_ROOT=/opt/dotnet \
    DOTNET_CLI_TELEMETRY_OPTOUT=1 \
    DOTNET_NOLOGO=1 \
    PATH=/opt/cargo/bin:/opt/dotnet:$PATH

# stable Rust, as CI's runners have it
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable \
        --no-modify-path \
    && chmod -R a+rX /opt/rustup /opt/cargo

# .NET 10, for Aetheris
RUN curl -sSfL https://dot.net/v1/dotnet-install.sh | bash -s -- --channel 10.0 \
        --install-dir /opt/dotnet

# the Python kernels, as kernels.json's `build` installs them (there they are then already met)
RUN python3 -m pip install --no-cache-dir cadquery-ocp usd-brep==1.0.1
