defmodule AppSerTest do
  use ExUnit.Case
  doctest AppSer

  test "AppSer.sum" do
    assert AppSer.sum(~s([{"value": 10}, {"value": 30}])) == {:ok, 40}
  end

  test "AppSerNif.sum" do
    assert AppSerNif.sum(~s([{"value": 10}, {"value": 30}])) == {:ok, 40}
  end
end
