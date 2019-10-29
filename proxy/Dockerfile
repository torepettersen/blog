FROM nginx:1-alpine

ARG CF_Key
ARG CF_Email

RUN apk add --no-cache curl openssl && \
    mkdir -p /etc/nginx/ssl/cloudmaker  && \
    wget -O -  https://get.acme.sh | sh && \
    ~/.acme.sh/acme.sh --issue --dns dns_cf -d cloudmaker.dev -d www.cloudmaker.dev && \
    ~/.acme.sh/acme.sh --installcert -d cloudmaker.dev \
        --keypath /etc/nginx/ssl/cloudmaker/ssl.key \
        --fullchainpath /etc/nginx/ssl/cloudmaker/ssl.crt

COPY nginx.conf /etc/nginx/nginx.conf

CMD /bin/sh -c 'while :; ~/.acme.sh/acme.sh --renew -d cloudmaker.dev -d www.cloudmaker.dev; do sleep 12h; nginx -s reload; done & nginx -g "daemon off;"'