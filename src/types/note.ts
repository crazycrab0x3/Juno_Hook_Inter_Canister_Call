export interface Note {
  name: string
  price_cents: number
  currency: string
}

export interface UpdateNote {
  name: string
  price_cents: number
  currency: string
  price_usd_cents: string
}
