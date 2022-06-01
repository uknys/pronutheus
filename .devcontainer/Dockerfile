FROM ubuntu:20.04

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/opt/openwrt/toolchain-mips_34kc+dsp_gcc-4.8-linaro_uClibc-0.9.33.2/bin:$PATH

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        build-essential pkg-config ca-certificates git curl libssl-dev sshpass && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* 

# Rustup
RUN curl -sSL --output /tmp/rustup-init "https://static.rust-lang.org/rustup/archive/1.24.3/x86_64-unknown-linux-gnu/rustup-init" && \
    chmod +x /tmp/rustup-init && \
    /tmp/rustup-init -y --no-modify-path --profile minimal --default-toolchain nightly && \
    rm /tmp/rustup-init && \
    chmod -R a+w ${RUSTUP_HOME} ${CARGO_HOME}

RUN (curl -sSL https://github.com/rust-analyzer/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | \
    gunzip -c - > /usr/local/bin/rust-analyzer) && \
    chmod +x /usr/local/bin/rust-analyzer

RUN rustup component add rust-src rustfmt clippy

RUN cargo install xargo

# Openwrt Toolchain
RUN curl -ssL --output /tmp/OpenWrt-Toolchain-lantiq-for-mips_34kc+dsp-gcc-4.8-linaro_uClibc-0.9.33.2.tar.bz2 "https://archive.openwrt.org/barrier_breaker/14.07/lantiq/xway/OpenWrt-Toolchain-lantiq-for-mips_34kc+dsp-gcc-4.8-linaro_uClibc-0.9.33.2.tar.bz2" && \
    tar xvf /tmp/OpenWrt-Toolchain-lantiq-for-mips_34kc+dsp-gcc-4.8-linaro_uClibc-0.9.33.2.tar.bz2 -C /opt && \
    mv /opt/OpenWrt-Toolchain-lantiq-for-mips_34kc+dsp-gcc-4.8-linaro_uClibc-0.9.33.2 /opt/openwrt