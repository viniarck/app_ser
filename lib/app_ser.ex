defmodule AppSer do
  def sum(v) do
    values = Jason.decode!(v)
    {:ok, Enum.reduce(values, 0, fn x, acc -> Map.get(x, "value") + acc end)}
  end
end
