df <- read.csv("blocked_urls.csv")
plot(ecdf(df[,"default"]),
     cex=0,
     main="Default vs. Fawkes",
     xlab="# of blocked URLs",
     ylab="CDF",
     col="red")

lines(ecdf(df[,"fawkes_static"]),
      cex=0,
      col="blue")

legend('bottomright', 
       legend=c("Default","Fawkes Static"),  # text in the legend
       col=c("red","blue"),  # point colors
       pch=15)  # specify the point type to be a square

############################################################
plot(ecdf(df[,"default_per"]),
     xlim=c(0,100),
     main="Default vs. Fawkes Percentage",
     xlab="% of blocked URLs",
     ylab="CDF",
     col="red")

lines(ecdf(df[,"fawkes_static_per"]),
#      cex=0,
      col="blue")

legend('bottomright', 
       legend=c("Default","Fawkes Static"),  # text in the legend
       col=c("red","blue"),  # point colors
       pch=15)  # specify the point type to be a square

