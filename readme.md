# System Metrics
## About the Project
System-metrics is used to collect and monitor system utilization details.
It has 3 level of monitoring:
1. System level
  <p> Used to collect system level details like ram usage, cpu usage, disk usage etc.</p>
2. Service level
  <p>Used to monitor services in the system, like memory usage of the service.</p>
3. Storage level 
  <p>Used to monitor the storage of specific paths</p>

All this details will be stored into influxdb database,  

<figure>
<img src="flowchart/system_metrics.png">
</figure>
