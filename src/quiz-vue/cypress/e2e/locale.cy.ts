describe('Locale Test', () => {
  it('visits the about page', () => {
    cy.visit('/about')
    cy.contains('.mt-4 > :nth-child(1)', 'Hi')
  })

  it('change the locales', () => {
    cy.get('.navbar-end > .btn').click()
    cy.contains('.mt-4 > :nth-child(1)', 'Hai')
  })
})
