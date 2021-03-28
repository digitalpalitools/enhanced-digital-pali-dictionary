/* eslint-disable no-restricted-syntax */
/* eslint-disable no-param-reassign */
// eslint-disable-next-line max-len
const paliCharacterOrder = 'a,ā,i,ī,u,ū,e,o,k,kh,g,gh,ṅ,c,ch,j,jh,ñ,ṭ,ṭh,ḍ,ḍh,ṇ,t,th,d,dh,n,p,ph,b,bh,m,y,r,l,v,s,h,ḷ,ṃ,0,1,2,3,4,5,6,7,8,9, '
  .split(',')
  .reduce((a: any, c, i) => {
    a[c] = i
    return a
  }, {})

const paliDoubles = 'kh|gh|ch|jh|ṭh|ḍh|th|dh|ph|bh'.split('|').reduce((a: any, c) => {
  a[c] = true
  return a
}, {})

const paliDoublesRegExp = /(kh|gh|ch|jh|ṭh|ḍh|th|dh|ph|bh)/

const cached: any = {}

const toPaliCharacterArray = (paliText: string) => {
  const c = cached[paliText]
  if (c) {
    return c
  }
  const chunks = paliText.toLowerCase().split(paliDoublesRegExp)
  const characters = []

  for (const chunk of chunks) {
    if (chunk.length === 1 || (chunk.length === 2 && paliDoubles[chunk])) {
      characters[characters.length] = chunk
    } else {
      for (const chr of chunk) {
        characters[characters.length] = chr
      }
    }
  }
  cached[paliText] = characters
  return characters
}

export const paliComparator = (a: string, b: string) => {
  const ac = toPaliCharacterArray(a)
  const bc = toPaliCharacterArray(b)
  const bl = Math.min(ac.length, bc.length)
  for (let i = 0; i < bl; i += 1) {
    const aci = ac[i]
    const bci = bc[i]
    const i1 = paliCharacterOrder[aci]
    const i2 = paliCharacterOrder[bci]
    if (i1 === undefined && i2 === undefined) {
      if (aci !== bci) {
        return aci.localeCompare(bci)
      }
    } else if (i1 === undefined) {
      return aci[0] - bci
    } else if (i2 === undefined) {
      return aci - bci[0]
    } else if (i1 !== i2) {
      return i1 - i2
    }
  }
  return ac.length - bc.length
}
