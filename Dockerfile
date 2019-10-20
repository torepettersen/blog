FROM node:12-alpine as build

WORKDIR /app

COPY ./package.json /app
RUN npm install

COPY ./ /app
RUN npm run build
RUN ls /app/dist


FROM nginx:1-alpine

ARG CF_Key
ARG CF_Email

RUN apk add --no-cache curl openssl && \
    mkdir -p /etc/nginx/ssl/cloudmaker  && \
    wget -O -  https://get.acme.sh | sh && \
    ~/.acme.sh/acme.sh --issue --dns dns_cf -d cloudmaker.dev && \
    ~/.acme.sh/acme.sh --installcert -d cloudmaker.dev \
        --keypath /etc/nginx/ssl/cloudmaker/ssl.key \
        --fullchainpath /etc/nginx/ssl/cloudmaker/ssl.crt && \
    mkdir /app
COPY --from=build /app/dist /app
COPY nginx.conf /etc/nginx/nginx.conf