describe('Locale Test', () => {
  it('click the locale button changes the language', () => {
    cy.visit('/about')
    cy.contains(':nth-child(2) > .mt-4', 'Browse your favourite pokemon')

    cy.get('.navbar-end > :nth-child(2)').click()
    cy.contains(':nth-child(2) > .mt-4', 'Cari pokemon kesukaanmu')
  })
})
