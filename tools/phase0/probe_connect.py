"""Phase 0, experiment 1. Read-only connection probe. Changes nothing in the game."""
import time

import krpc

conn = krpc.connect(name="ikria-probe-readonly", address="127.0.0.1", rpc_port=50000, stream_port=50001)
k = conn.krpc
print("server version:", k.get_status().version)
print("game scene:", k.current_game_scene if hasattr(k, "current_game_scene") else k.game_scene)
sc = conn.space_center
print("game mode:", sc.game_mode)
print("ut:", sc.ut)
print("bodies:", len(sc.bodies))
procs = [p for p in dir(k) if "tick" in p.lower()]
print("KRPC tick procedures in client:", procs)
status = k.get_status()
for f in ("rpc_rate", "rpcs_executed", "max_time_per_update", "adaptive_rate_control",
          "one_rpc_per_update", "time_per_rpc_update", "poll_rpc_update", "exec_rpc_update",
          "stream_rpc_rate", "stream_rpcs_executed", "time_per_stream_update"):
    if hasattr(status, f):
        print(f"status.{f}:", getattr(status, f))
t0 = time.perf_counter()
n = 200
for _ in range(n):
    sc.ut
dt = time.perf_counter() - t0
print(f"{n} sequential calls of ut: {dt*1000:.1f} ms total, {dt/n*1000:.3f} ms each")
conn.close()
