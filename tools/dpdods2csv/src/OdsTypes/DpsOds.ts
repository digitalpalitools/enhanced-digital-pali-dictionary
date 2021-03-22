import * as Ods from '../OdsProcessor'

/*
pāli:
(!)
POS
Grammar
Derived from
Neg
Verb
Trans
Case
Meaning in english
in native language
pali root
base
constraction
sanskrit
sk root
Comments
sourse 1
sentence from the pāli cannon 1
sutta name pali 1
source 2
sentence 2
chant name 2
Chapter
Test
*/

const shortName = 'dps'

class PaliWord implements Ods.PaliWordBase {
  readonly record: string[]

  constructor(record: string[]) {
    this.record = record
  }

  get pali() {
    return this.record[0]
  }

  get bang() {
    return this.record[1]
  }

  get pos() {
    return this.record[2]
  }

  get grammar() {
    return this.record[3]
  }

  get derivedFrom() {
    return this.record[4]
  }

  get neg() {
    return this.record[5]
  }

  get verb() {
    return this.record[6]
  }

  get trans() {
    return this.record[7]
  }

  get case() {
    return this.record[8]
  }

  get inEnglish() {
    return this.record[9]
  }

  get inRussian() {
    return this.record[10]
  }

  get paliRoot() {
    return this.record[11]
  }

  get base() {
    return this.record[12]
  }

  get construction() {
    return this.record[13]
  }

  get sanskrit() {
    return this.record[14]
  }

  get sanskritRoot() {
    return this.record[15]
  }

  get comments() {
    return this.record[16]
  }

  get source1() {
    return this.record[17]
  }

  get sentence1() {
    return this.record[18]
  }

  get sutta1() {
    return this.record[19]
  }

  get source2() {
    return this.record[20]
  }

  get sentence2() {
    return this.record[21]
  }

  get sutta2() {
    return this.record[22]
  }

  get chapter() {
    return this.record[23]
  }

  get test() {
    return this.record[24]
  }

  isValidWord() {
    return !!this.pali
  }

  groupId() {
    return Ods.makeGroupId(this.pali)
  }

  toCsvRow(): string {
    return Ods.toCsvRow(this.record)
  }

  createTocSummary(): string {
    return `<li><a href="#${this.tocId()}">${this.pali}</a>: ${this.pos}. ${this.inEnglish}</li>`
  }

  createWordData(): string {
    /* eslint-disable */ // ESList is unable to handle the complicated templating + concatenation
    const html = `
  <hr />
  <div>
    <h4 id="${this.tocId()}">${this.pali}</h4>
    <table class="word-info-table-${shortName}">
      <tbody>` +
      (this.grammar && `<tr><td>Grammar</td><td><span>${this.pos}, ${this.grammar}` + (this.derivedFrom && `, from ${this.derivedFrom}`) + (this.verb && `, ${this.verb}`) + (this.neg && `, ${this.neg}`) + (this.trans && `, ${this.trans}`) + (this.case && ` (${this.case})`) + `</span></td></tr>`) +
      (this.inEnglish && `<tr><td>English</td><td><span><strong>${this.inEnglish}</strong></span></td></tr>`) +
      (this.inRussian && `<tr><td>Russian</td><td><span><strong>${this.inRussian}</strong></span></td></tr>`) +
      (this.paliRoot && `<tr><td>Pāli Root</td><td><span>${this.paliRoot}</span></td></tr>`) +
      (this.base && `<tr><td>Base</td><td><span>${this.base}</span></td></tr>`) +
      (this.construction && `<tr><td>Construction</td><td><span>${this.construction}</span></td></tr>`) +
      (this.sanskrit && `<tr><td>Sanskrit</td><td><span>${this.sanskrit}</span></td></tr>`) +
      (this.sanskritRoot && `<tr><td>Sanskrit Root</td><td><span>${this.sanskritRoot}</span></td></tr>`) +
      (this.comments && `<tr><td>Commentary</td><td><span>${this.comments}</span></td></tr>`) +
      `</tbody>
    </table>`
    +
    (this.sentence1 && `<br /><span>${this.sentence1}</span><br />`) +
    (this.source1 && `<span class="sutta-source-${shortName}"><i>${this.source1} ${this.sutta1}</i></span><br />`) +
    (this.sentence2 && `<br /><span>${this.sentence2}</span><br />`) +
    (this.source2 && `<span class="sutta-source-${shortName}"><i>${this.source2} ${this.sutta2}</i></span><br />`) +
    (this.chapter && `<span class="sutta-source-${shortName}"><i>${this.chapter}</i></span><br /><br />`) +
  `</div>`
    /* eslint-enable */

    return html
  }

  tocId = () => `${this.pali.replace(/\s/g, '')}-${shortName}}`

  includeInDictionary = () => !!this.inEnglish && !!this.inRussian

  includeInRootCsv = () => true

  sortKey = () => Ods.padTrailingNumbers(this.pali)
}

const createPaliWord: Ods.PaliWordFactory = x => new PaliWord(x)

export const dpsOds: Ods.OdsType = {
  name: 'Devamitta Pāli Study',
  shortName,
  author: 'Devamitta Bhikkhu',
  description: 'A detailed Pāli language word lookup',
  accentColor: 'green',
  icon:
    // eslint-disable-next-line max-len
    'iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAADSUlEQVRYR+3WW4hVVRzH8c86muGtKBMrMjW7QUqRIEpQaD2UEEG+Si9JEUEQRaHGnHUmHVEriEAq7EEqegh8mAojKiiCILrQxZIKKaO0IhMds8aZs2Kvs2fcOmcuNQ++zHo6Z6///q/v/q3f/79WcIZHOMPrmwCYUCBYZ5bppg2a8YQkOCzqaWvQTeZIppwy1+eY6NCwhq67VLLQZL2a9ogOD8QG0YtY0+blfZKdDtniGf8MzkcfYtmQ+ORHQZfo+cG5jebqsxMrKvEn8BweFPVVAXrxMSZhHi4sX+oW3TEEIPhDyjCFGldifhlzb4aIJuMzLEKP5AM152Fp1pjNovVVgIOii3KSIqChE4+VSW8WvZt/n1TgfdFN5XyR51ncg/2ieaJbsRvFhy0WfZtj69YLNmUo5rQHaC00TXJEyIpsEz0yAgANK6QSss9lznKn5Al8Ibq2omDhtz+zck0rhwdoQfyES7BLtHpEgJil/rJc6IbSJ0/iZ9HcrOvAiN6T/KapczSAvbgKb4puGwXgenySY2qW6jdb8Eb+H6xV90K7KhkN4BtcPSaADner2SHpF1yAvwR7JQvQh/tEO06HGA/Ap2rW6nc2rlPTKZkt2aVRblfdMjVvSWaWC28UdVS3YzwAQxUN9khuER0cnOy0RNPrlbIu+s5aMVeH8QD8WpZZkeeo5COzvOqBStMaoHjcAv25JAs/FaMbq09vRCf7wEm3jt0D7RxWfdZlll6vYXn5eKvo0fEoUG1Eoy3fmt9muh5vC5aVZl00MkDdd4LLJbs1rBqxDE9HeMpUR92Y+0i1BOsuxveCqWiMpsCBbJ7kZY3ywGrfiocq0GGdmi7sEy08JSB6Byslr4zUimfkYzmZJNmgkZMNdxYMBWi4XdItSJrma9g/GBRzg1pVfNjwAHUNQUdOECzWYc9/Aij2+5hfcA5eUndXztXagq8F5+KhKkBx5hfS1PLlIbiiJN4uur9CP3AfGN2E0cPZeq1RtPXijrFcyMfyAX+7ZqQLSWGU7ZKnRc3/BVD0mbp1gg1Ubl3J54I1oq8KgPNNzo5sjeM44YitjratrS6zNU1xTK/Nfh9T/W0x03FLMAM/FAsPvDdxK55QYEKBfwGdwU6OcOXRFwAAAABJRU5ErkJggg==',
  paliWordFactory: createPaliWord,
}
