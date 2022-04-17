FROM rust:1.56

RUN mkdir /root/walkietalkie
RUN apt install bash

RUN rustup default nightly

WORKDIR /root/walkietalkie

COPY ./ /root/walkietalkie

VOLUME /root/walkietalkie

EXPOSE 14014