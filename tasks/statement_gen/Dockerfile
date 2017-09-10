FROM money-map/api
MAINTAINER Brandon Powers "bpowers1215@gmail.com"

# Install dependencies
RUN apt-get update && apt-get -y install cron

# Copy crontab file in the cron directory
COPY crontab /etc/cron.d/statement-gen-cron

# Give execution rights on the cron job
RUN chmod 0644 /etc/cron.d/statement-gen-cron

# Create the log file to be able to run tail
RUN touch /var/log/cron.log

# Copy task script
COPY start-task.sh /bin/start-task.sh

# Give execution rights on the task script
RUN chmod 0744 /bin/start-task.sh

WORKDIR /var/lib/money_map/statement_gen

# Run the command on container startup
#CMD cron && tail -f /var/log/cron.log
CMD ["cargo", "run"]
