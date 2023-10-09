describe('Question Limit', () => {
  it('test the question limit', () => {
    cy.visit('/')
    cy.get('.btn-primary').click()
    cy.contains(
      '#app > :nth-child(2) > :nth-child(1) > :nth-child(1)',
      '0 / 10'
    )
    cy.contains(
      '#app > :nth-child(2) > :nth-child(1) > :nth-child(2)',
      '0 / 10'
    )

    // visits the result page
    cy.visit('/result')
    cy.contains('.mt-2 > :nth-child(1)', 'Easy')
    cy.contains('.mt-2 > :nth-child(2)', '10')
  })
})
