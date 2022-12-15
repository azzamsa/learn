describe('Locale Test', () => {
  it('toggle the locale button', () => {
    cy.visit('/about')
    cy.contains('.mt-4 > :nth-child(1)', 'Hi')

    // change the locales
    cy.get('.navbar-end > .btn').click()
    cy.contains('.mt-4 > :nth-child(1)', 'Hai')
  })
})
