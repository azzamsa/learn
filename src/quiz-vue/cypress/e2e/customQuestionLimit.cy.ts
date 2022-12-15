describe('Question Limit', () => {
  it('test the quiz question limit', () => {
    // visits the index page
    cy.visit('/')
    cy.get('.btn-secondary').click()
    cy.get('.mt-4 > :nth-child(2)').select('20')

    cy.get('.btn-primary').click()
    cy.contains(
      '#app > :nth-child(2) > :nth-child(1) > :nth-child(1)',
      '0 / 20'
    )
    cy.contains(
      '#app > :nth-child(2) > :nth-child(1) > :nth-child(2)',
      '0 / 20'
    )

    // doing the quiz
    for (let n = 0; n < 20; n++) {
      /* eslint-disable cypress/no-unnecessary-waiting */
      cy.wait(3000).get('.card-actions > .btn').click()
    }

    // check the result page
    cy.contains('.mt-2 > :nth-child(2)', '20')
  })
})
