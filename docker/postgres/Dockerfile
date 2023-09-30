FROM postgres

COPY ./*.sql /sql_scirpts/
COPY ./*.sh /docker-entrypoint-initdb.d/

RUN chmod a+r -R /sql_scirpts