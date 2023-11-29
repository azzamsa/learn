describe('Question Limit', () => {
  it('test the question limit', () => {
    cy.visit('/')
    cy.get('.flex > .btn-primary').click()

    // check `Questions`
    cy.contains(
      '#app > :nth-child(2) > :nth-child(1) > :nth-child(1)',
      '0 / 10',
    )
    // check `Correct Answer`
    cy.contains(
      '#app > :nth-child(2) > :nth-child(1) > :nth-child(2)',
      '0 / 10',
    )

    // doing the quiz
    for (let n = 0; n < 10; n++) {
      // need hard `wait`, `{ timeout: n }` doesn't work
      cy.get('.card-actions > .btn').click()
    }

    // in the result page
    cy.contains('.mt-2 > :nth-child(2)', '10')
  })
})
