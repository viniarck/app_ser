{:ok, contents} = File.read("values.json")

Benchee.run(
  %{"AppSer" => fn -> AppSer.sum(contents) end, "AppSerNif" => fn -> AppSerNif.sum(contents) end},
  time: 60
)
