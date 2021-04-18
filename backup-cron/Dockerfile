FROM amazon/aws-cli

RUN yum install sqlite -y

COPY backup.sh /usr/local/bin/backup.sh
RUN chmod +x /usr/local/bin/backup.sh

RUN mkdir /var/local/backups

CMD /usr/local/bin/backup.sh
ENTRYPOINT ["/usr/local/bin/backup.sh"]

