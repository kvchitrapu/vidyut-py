ARG VIDYUT_VER=0.2.0

# ####################################
# Stage 1: build the base image with Rust and Python dependencies
FROM python:3.11.1-slim-buster as base
ENV PYTHONFAULTHANDLER=1 \
    PYTHONHASHSEED=random \
    PYTHONUNBUFFERED=1

# ####################################
# Stage 2: build the Rust code along with python bindings
FROM base as build-vidyut-py
ARG VIDYUT_VER=0.2.0
WORKDIR /usr/src/
COPY . .

# Install wget
RUN apt-get update && apt-get install --yes --no-install-recommends make git wget curl unzip build-essential 
# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

RUN echo "Diabled vidyut build for now. "
WORKDIR /usr/local/vidyut

# download Vidyut data from the github repo
WORKDIR /var/vidyut/
RUN wget https://github.com/ambuda-org/vidyut-py/releases/download/${VIDYUT_VER}/data-${VIDYUT_VER}.zip && unzip data-${VIDYUT_VER}.zip
WORKDIR /usr/src/
RUN . "$HOME/.cargo/env" && export VIDYUT_DATA_DIR="/var/vidyut/data-${VIDYUT_VER}/" && make install

# ####################################f
# Stage 3: Prepare final image

# TODO: Find where the binaries are and just the binaries and not dev images.
FROM build-vidyut-py as final 
ARG VIDYUT_VER=0.2.0
RUN echo "************* Building FINAL ${BUILD_TYPE}*****************"
COPY --from=build-vidyut-py /usr/local/vidyut /var/local/vidyut
COPY --from=build-vidyut-py /var/vidyut/data-${VIDYUT_VER} /var/vidyut/data-${VIDYUT_VER}
WORKDIR /usr/local/vidyut
ENV VIDYUT_DATA_DIR="/var/vidyut/data-${VIDYUT_VER}/"
ENTRYPOINT [ "pytest test/unit/" ]
RUN echo "************* Vidyut dev image is ready! *****************"
# ####################################

