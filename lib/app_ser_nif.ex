defmodule AppSerNif do
  use Rustler, otp_app: :app_ser, crate: "app_ser_nif"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def sum(_a), do: :erlang.nif_error(:nif_not_loaded)
end
