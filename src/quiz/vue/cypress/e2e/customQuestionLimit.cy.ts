describe('Question Limit', () => {
  it('test the quiz question limit', () => {
    // visits the index page
    cy.visit('/')
    cy.get('.btn-secondary').click()
    cy.get('.mt-4 > :nth-child(2)').select('20')

    cy.get('.flex > .btn-primary').click()
    // check `Questions`
    cy.contains(
      '#app > :nth-child(2) > :nth-child(1) > :nth-child(1)',
      '0 / 20',
    )
    // check `Correct Answer`
    cy.contains(
      '#app > :nth-child(2) > :nth-child(1) > :nth-child(2)',
      '0 / 20',
    )

    // doing the quiz
    for (let n = 0; n < 20; n++)
      cy.get('.card-actions > .btn').click()

    // check the result page
    cy.contains('.mt-2 > :nth-child(2)', '20')
  })
})
