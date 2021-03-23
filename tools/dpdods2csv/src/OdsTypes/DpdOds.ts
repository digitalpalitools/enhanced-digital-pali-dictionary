import * as Ods from '../OdsProcessor'

/*
"Pāli2"
"Fin"
"POS"
"Grammar"
"Derived from"
"Neg"
"Verb"
"Trans"
"Case"
"Meaning IN CONTEXT"
"Sanskrit"
"Sk Root"
"Family"
"Pāli Root"
"V"
"Grp"
"Sgn"
"Root Meaning"
"Base"
"Construction"
"Derivative"
"Suffix"
"Compound"
"Compound Construction"
"Source1"
"Sutta1"
"Example1"
"Source 2"
"Sutta2"
"Example 2"
"Antonyms"
"Synonyms – different word"
"Variant – same constr or diff reading"
"Commentary"
"Notes"
"Stem"
"Pattern"
"Buddhadatta"
"2"
*/

const shortName = 'dpd'

class PaliWord implements Ods.PaliWordBase {
  readonly record: string[]

  constructor(record: string[]) {
    this.record = record
  }

  get pali1() {
    return this.record[0]
  }

  get pali2() {
    return this.record[1]
  }

  get fin() {
    return this.record[2]
  }

  get pos() {
    return this.record[3]
  }

  get grammar() {
    return this.record[4]
  }

  get derivedFrom() {
    return this.record[5]
  }

  get neg() {
    return this.record[6]
  }

  get verb() {
    return this.record[7]
  }

  get trans() {
    return this.record[8]
  }

  get case() {
    return this.record[9]
  }

  get inEnglish() {
    return this.record[10]
  }

  get sanskrit() {
    return this.record[11]
  }

  get sanskritRoot() {
    return this.record[12]
  }

  get family() {
    return this.record[13]
  }

  get paliRoot() {
    return this.record[14]
  }

  get v() {
    return this.record[15]
  }

  get grp() {
    return this.record[16]
  }

  get sgn() {
    return this.record[17]
  }

  get rootMeaning() {
    return this.record[18]
  }

  get base() {
    return this.record[19]
  }

  get construction() {
    return this.record[20]
  }

  get derivative() {
    return this.record[21]
  }

  get suffix() {
    return this.record[22]
  }

  get compound() {
    return this.record[23]
  }

  get compoundConstruction() {
    return this.record[24]
  }

  get source1() {
    return this.record[25]
  }

  get sutta1() {
    return this.record[26]
  }

  get example1() {
    return this.record[27]
  }

  get source2() {
    return this.record[28]
  }

  get sutta2() {
    return this.record[29]
  }

  get example2() {
    return this.record[30]
  }

  get antonyms() {
    return this.record[31]
  }

  get synonyms() {
    return this.record[32]
  }

  get variant() {
    return this.record[33]
  }

  get commentary() {
    return this.record[34]
  }

  get notes() {
    return this.record[35]
  }

  get stem() {
    return this.record[36]
  }

  get pattern() {
    return this.record[37]
  }

  get buddhadatta() {
    return this.record[38]
  }

  get two() {
    return this.record[39]
  }

  isValidWord() {
    return !!this.pali1
  }

  groupId() {
    return Ods.makeGroupId(this.pali1)
  }

  toCsvRow(): string {
    return Ods.toCsvRow(this.record)
  }

  createTocSummary(): string {
    return `<li><a href="#${this.tocId()}">${this.pali1}</a>: ${this.pos}. ${this.inEnglish}</li>`
  }

  createWordData(): string {
    /* eslint-disable */ // ESList is unable to handle the complicated templating + concatenation
    const html = `
  <hr />
  <div>
    <h4 id="${this.tocId()}">${this.pali1}</h4>
    <table class="word-info-table-${shortName}">
      <tbody>` +
      `<tr><td>Pāli</td><td><span>${this.pali2}</span></td></tr>` +
      (this.grammar && `<tr><td>Grammar</td><td><span>${this.grammar}` + (this.verb && `, ${this.verb}`) + (this.neg && `, ${this.neg}`) + (this.trans && `, ${this.trans}`) + (this.case && ` (${this.case})`) + `</span></td></tr>`) +
      (this.inEnglish && `<tr><td>English</td><td><span><strong>${this.inEnglish}</strong></span></td></tr>`) +
      (this.family && `<tr><td>Family</td><td><span>${this.family}</span></td></tr>`) +
      (this.paliRoot && `<tr><td>Root</td><td><span>${this.paliRoot}<sup>${this.v}</sup>${this.grp} ${this.sgn} (${this.rootMeaning})</span></td></tr>`) +
      (this.base && `<tr><td>Base</td><td><span>${this.base}</span></td></tr>`) +
      (this.construction && `<tr><td>Construction</td><td><span>${this.construction}</span></td></tr>`) +
      (this.derivative && `<tr><td>Derivative</td><td><span>${this.derivative} (${this.suffix})</span></td></tr>`) +
      (this.compound && `<tr><td>Compound</td><td><span>${this.compound} (${this.compoundConstruction})</span></td></tr>`) +
      (this.antonyms && `<tr><td>Antonym</td><td><span>${this.antonyms}</span></td></tr>`) +
      (this.synonyms && `<tr><td>Synonym</td><td><span>${this.synonyms}</span></td></tr>`) +
      (this.variant && `<tr><td>Variant</td><td><span>${this.variant}</span></td></tr>`) +
      (this.sanskrit && `<tr><td>Sanskrit</td><td><span>${this.sanskrit}</span></td></tr>`) +
      (this.sanskritRoot && `<tr><td>Sanskrit Root</td><td><span>${this.sanskritRoot}</span></td></tr>`) +
      (this.commentary && `<tr><td>Commentary</td><td><span>${this.commentary}</span></td></tr>`) +
      (this.notes && `<tr><td>Notes</td><td><span>${this.notes}</span></td></tr>`) +
      `</tbody>
    </table>
    <br />` +
    (this.example1 && `<span>${this.example1}</span><br />`) +
    (this.source1 && `<span class="sutta-source-${shortName}"><i>${this.source1} ${this.sutta1}</i></span><br /><br />`) +
    (this.example2 && `<span>${this.example2}</span><br />`) +
    (this.source2 && `<span class="sutta-source-${shortName}"><i>${this.source2} ${this.sutta2}</i></span><br /><br />`) +
  `</div>`
    /* eslint-enable */

    return html
  }

  tocId = () => `${this.pali1.replace(/\s/g, '_')}_${shortName}`

  includeInDictionary = () => !!this.inEnglish

  includeInRootCsv = () => !!this.paliRoot

  sortKey = () => Ods.padTrailingNumbers(this.pali1)
}

const createPaliWord: Ods.PaliWordFactory = x => new PaliWord(x)

export const dpdOds: Ods.OdsType = {
  name: 'Digital Pāli Dictionary (DPD)',
  shortName,
  author: 'A Pāli Instructor',
  description: 'A detailed Pāli language word lookup',
  accentColor: 'orange',
  icon:
    // eslint-disable-next-line max-len
    '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',
  paliWordFactory: createPaliWord,
}
