FROM node:12-alpine as build

WORKDIR /app

COPY ./package.json /app
RUN npm install

COPY ./ /app
RUN npm run build
RUN ls /app/dist


FROM nginx:1-alpine

RUN mkdir /app
COPY --from=build /app/dist /app
COPY nginx.conf /etc/nginx/nginx.conf