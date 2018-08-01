FROM alpine:3.5

MAINTAINER fehguy

ENV VERSION "v2.2.10"
ENV FOLDER "swagger-ui-2.2.10"
ENV PORT 8080
ENV HOST ""
ENV SCHEME "http"
ENV BASE_PATH "/"

RUN apk add --update nginx
RUN mkdir -p /run/nginx

COPY nginx.conf /etc/nginx/

# copy swagger files to the `/js` folder
ADD ./dist/* /usr/share/nginx/html/
ADD ./docker-run.sh /usr/share/nginx/
ADD ./iptmnet_spec.json /usr/share/nginx/html/

EXPOSE 8080

CMD ["sh", "/usr/share/nginx/docker-run.sh"]
