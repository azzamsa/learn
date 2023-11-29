describe('custom Level', () => {
  it('test the quiz custom level', () => {
    cy.visit('/')
    cy.get('.btn-secondary').click()
    cy.get('.mt-4 > :nth-child(1)').select('Medium')

    cy.get('.flex > .btn-primary').click()

    // doing the quiz
    for (let n = 0; n < 10; n++) {
      // need hard `wait`, `{ timeout: n }` doesn't work
      cy.get('.card-actions > .btn').click()
    }

    cy.contains('.mt-2 > :nth-child(1)', 'Medium')
  })
})
