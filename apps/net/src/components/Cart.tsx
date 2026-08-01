import { useSyncExternalStore } from "react"
import { drop, listen, snapshot } from "../lib/orders"
import { HELP } from "../lib/site"

export function Cart() {
  const orders = useSyncExternalStore(listen, snapshot)
  return (
    <div className="cart">
      <p>cart</p>
      <ul className="orders">
        {orders.map((order, at) => (
          <li key={at}>
            <span className="name">{`${order.product} ${order.label}`}</span>
            <span className="cost">{`€${String(order.price)}`}</span>
            <button className="drop" type="button" aria-label="remove" onClick={() => drop(at)}>
              {"×"}
            </button>
          </li>
        ))}
      </ul>
      <p className={orders.length > 0 ? "empty gone" : "empty"}>No pre-orders yet.</p>
      <a href={HELP}>Need a hand?</a>
    </div>
  )
}
