FROM fedora:latest

RUN dnf makecache

RUN dnf install -y \
    inn \
    passwd \
    httpd-tools

# TODO(craft): We should clear the DNF cache

RUN useradd newsreader \
    && echo "readthenews" | passwd newsreader --stdin

# TODO(sec): We setuid so the news user can authenticate against the shadow DB. Deffo not for production
RUN chmod u+s /usr/libexec/news/auth/passwd/ckpasswd

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini

# Copy configuration
COPY root /
RUN chown -R news:news /var/lib/news

USER news
ENV PATH "$PATH:/usr/libexec/news"

# Create the history database
RUN makedbz -i -o
# Create some test groups
RUN innd \
    && sleep 2 \
    && ctlinnd newgroup test.music \
    && ctlinnd newgroup test.jokes \
    && ctlinnd shutdown "created test groups"


# innd only seems to exit with SIGQUIT
STOPSIGNAL SIGQUIT
ENTRYPOINT ["/tini", "--"]
CMD [ "/usr/libexec/news/innd", "-d" ]
