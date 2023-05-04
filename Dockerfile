FROM debian

# Install base packages
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y build-essential cmake curl nano ca-certificates unzip

RUN apt-get update \
 && apt-get install --assume-yes --no-install-recommends --quiet \
        python3 \
        python3-pip \
        pkg-config \
        libfontconfig1-dev \
        libfontconfig \
 && apt-get clean all

# Setup Python
RUN pip install --no-cache --upgrade pip setuptools
RUN pip --version 
RUN pip install maturin flask numpy pandas matplotlib seaborn openpyxl pycel gspread flask_cors waitress

# Setup Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Node.js
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs

RUN npm install -g cross-env

WORKDIR /home/pace

COPY ["Cargo.toml", "Cargo.lock", "./"]

COPY benchmarks/pace ./benchmarks/pace
COPY example ./example

COPY pace ./pace
COPY bindings/python ./bindings/python

RUN maturin build --manifest-path bindings/python/Cargo.toml -o build --interpreter python3.9 --release
RUN pip install build/nersent_pace_py-0.0.1-cp39-cp39-manylinux_2_28_x86_64.whl --force-reinstall

COPY api ./api

EXPOSE 80

# listen on port 8080
# CMD tail -f /dev/null

CMD ["cross-env", "PYTHONPATH=\"./\"", "python3", "api/src/app.py"]

# FROM rust

# WORKDIR /home/pace

# COPY ["pace", "Cargo.toml", "Cargo.lock", "./"]

# CMD ["/bin/docker-program"]
